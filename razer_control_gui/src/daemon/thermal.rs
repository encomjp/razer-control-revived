//! Pure thermal-safety policy for the Razer Blade 16 2025 (PID 0x02C6).
//!
//! Every function here is a pure decision: no HID I/O, no daemon state. The
//! RPM ranges are Synapse product-710 configuration defaults and are the
//! permanent validation basis: the EC does not implement `0x86` Get Thermal
//! Fan Information (verified NOT_SUPPORTED on the physical unit), so no
//! EC-reported limits exist to replace them. Hyperboost (7) and custom level
//! 3 (Extreme) are exposed: both passed the supervised latch validation on
//! the physical unit (2026-07-13). Hyperboost is AC-only and its UIs note
//! that Razer pairs it with the cooling pad.

#![allow(dead_code)]

pub const BLADE_16_2025_PID: u16 = 0x02c6;

/// Check whether a device PID string (as stored in laptops.json, e.g. "02C6")
/// matches the Blade 16 2025 — the only model whose EC protocol this module
/// validates. The daemon must call this before using any function in this
/// module; non-matching devices must continue using the generic code paths.
pub fn is_supported_pid(pid: &str) -> bool {
    u16::from_str_radix(pid.trim(), 16).is_ok_and(|p| p == BLADE_16_2025_PID)
}

/// EC fan zones as carried in the fan-ID byte of every 0x0d thermal command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanId {
    Cpu,
    Gpu,
}

impl FanId {
    pub const fn wire_value(self) -> u8 {
        match self {
            FanId::Cpu => 1,
            FanId::Gpu => 2,
        }
    }

    /// Map an EC fan-ID byte back to a zone. Only the two documented zones are
    /// recognized; every other byte is rejected rather than defaulted.
    pub const fn from_wire(byte: u8) -> Option<FanId> {
        match byte {
            1 => Some(FanId::Cpu),
            2 => Some(FanId::Gpu),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSource {
    Ac,
    Battery,
}

/// Performance modes the 02C6 EC recognizes. All decode; selectability is
/// per power domain (see `selectable_modes`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceMode {
    Balanced,
    /// The DC-domain slot of Balanced (wire 6): the EC partitions the mode
    /// namespace by power source, so Balanced latches as 0 on AC and 6 on
    /// battery. Observed on the validation unit (EC reported 6 after a
    /// battery boot) and corroborated by the hoenerc fork's evidence-tagged
    /// EC protocol reference for PID 02C6.
    BalancedBattery,
    MaximumPerformance,
    BatterySaver,
    Custom,
    Silent,
    Hyperboost,
}

impl PerformanceMode {
    pub const fn wire_value(self) -> u8 {
        match self {
            PerformanceMode::Balanced => 0,
            PerformanceMode::BalancedBattery => 6,
            PerformanceMode::MaximumPerformance => 2,
            PerformanceMode::BatterySaver => 3,
            PerformanceMode::Custom => 4,
            PerformanceMode::Silent => 5,
            PerformanceMode::Hyperboost => 7,
        }
    }
}

impl TryFrom<u8> for PerformanceMode {
    type Error = ThermalPolicyError;

    fn try_from(wire_value: u8) -> Result<PerformanceMode, ThermalPolicyError> {
        match wire_value {
            0 => Ok(PerformanceMode::Balanced),
            2 => Ok(PerformanceMode::MaximumPerformance),
            3 => Ok(PerformanceMode::BatterySaver),
            4 => Ok(PerformanceMode::Custom),
            5 => Ok(PerformanceMode::Silent),
            6 => Ok(PerformanceMode::BalancedBattery),
            7 => Ok(PerformanceMode::Hyperboost),
            _ => Err(ThermalPolicyError::UnknownMode { wire_value }),
        }
    }
}

/// A fan speed in RPM. As a validated fixed manual *target* it is never
/// constructed from an out-of-range request and is never zero (zero selects
/// firmware automatic control). The same newtype also carries a raw 0x88
/// tachometer *sample*, which may read zero when a fan has stopped;
/// `classify_manual_reading` treats such a zero as a distinct failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FanRpm(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RpmRange {
    pub min: u16,
    pub max: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalPolicyError {
    UnknownMode {
        wire_value: u8,
    },
    ModeNotSelectable {
        mode: PerformanceMode,
        source: PowerSource,
    },
    RpmOutOfRange {
        mode: PerformanceMode,
        requested_rpm: i32,
        range: RpmRange,
    },
    LevelOutOfRange {
        level: u8,
    },
}

impl std::fmt::Display for ThermalPolicyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThermalPolicyError::UnknownMode { wire_value } => {
                write!(
                    formatter,
                    "unknown performance-mode wire value {wire_value}"
                )
            }
            ThermalPolicyError::ModeNotSelectable { mode, source } => {
                write!(formatter, "mode {mode:?} is not selectable on {source:?}")
            }
            ThermalPolicyError::RpmOutOfRange {
                mode,
                requested_rpm,
                range,
            } => {
                write!(
                    formatter,
                    "requested {requested_rpm} RPM is outside {}..={} for mode {mode:?}",
                    range.min, range.max
                )
            }
            ThermalPolicyError::LevelOutOfRange { level } => {
                write!(
                    formatter,
                    "custom level {level} is not a recognized level (0-3)"
                )
            }
        }
    }
}

const AC_SELECTABLE_MODES: [PerformanceMode; 5] = [
    PerformanceMode::Balanced,
    PerformanceMode::Silent,
    PerformanceMode::MaximumPerformance,
    PerformanceMode::Custom,
    PerformanceMode::Hyperboost,
];

const BATTERY_SELECTABLE_MODES: [PerformanceMode; 2] = [
    PerformanceMode::BalancedBattery,
    PerformanceMode::BatterySaver,
];

/// The Balanced slot for a power domain: the EC partitions Balanced into wire 0
/// (AC) and wire 6 (battery), so "default to Balanced" must pick per source.
pub const fn balanced_for(source: PowerSource) -> PerformanceMode {
    match source {
        PowerSource::Ac => PerformanceMode::Balanced,
        PowerSource::Battery => PerformanceMode::BalancedBattery,
    }
}

/// Modes the daemon offers for the given power source. Hyperboost is AC-only:
/// it is an AC-domain wire value, and Razer pairs it with the cooling pad, so
/// the frontends label it accordingly.
pub fn selectable_modes(source: PowerSource) -> &'static [PerformanceMode] {
    match source {
        PowerSource::Ac => &AC_SELECTABLE_MODES,
        PowerSource::Battery => &BATTERY_SELECTABLE_MODES,
    }
}

pub fn is_mode_selectable(source: PowerSource, mode: PerformanceMode) -> bool {
    selectable_modes(source).contains(&mode)
}

/// Synapse product-710 configuration defaults. These are the permanent
/// validation basis: the 02C6 EC does not implement the 0x86 limits read
/// (verified NOT_SUPPORTED on hardware), so no EC-reported ranges exist.
pub const fn provisional_rpm_range(mode: PerformanceMode) -> RpmRange {
    match mode {
        PerformanceMode::Balanced | PerformanceMode::BalancedBattery | PerformanceMode::Silent => {
            RpmRange {
                min: 3400,
                max: 5200,
            }
        }
        PerformanceMode::MaximumPerformance | PerformanceMode::BatterySaver => RpmRange {
            min: 3300,
            max: 5400,
        },
        PerformanceMode::Custom => RpmRange {
            min: 4000,
            max: 5300,
        },
        PerformanceMode::Hyperboost => RpmRange {
            min: 3700,
            max: 5300,
        },
    }
}

/// Maximum accepted deviation between a fixed manual target and the 0x88
/// tachometer reading before a verification cycle counts as failed.
pub fn manual_tolerance(target_rpm: u16) -> u16 {
    500_u16.max(target_rpm / 4)
}

/// Validate a requested fixed fan speed against the mode's provisional range.
/// Zero selects firmware automatic control and is not range-checked. Nonzero
/// requests are never clamped: out-of-range requests are rejected outright.
pub fn validate_fixed_rpm(
    mode: PerformanceMode,
    requested_rpm: i32,
) -> Result<Option<FanRpm>, ThermalPolicyError> {
    if requested_rpm == 0 {
        return Ok(None);
    }
    let range: RpmRange = provisional_rpm_range(mode);
    if requested_rpm < i32::from(range.min) || requested_rpm > i32::from(range.max) {
        return Err(ThermalPolicyError::RpmOutOfRange {
            mode,
            requested_rpm,
            range,
        });
    }
    Ok(Some(FanRpm(requested_rpm as u16)))
}

/// Validate a custom CPU/GPU level for the 0x07 setter. Levels 0-3 pass;
/// level 3 (Extreme) passed its supervised latch validation on the physical
/// unit (2026-07-13, both zones, write + identical readback).
pub fn validate_custom_level(level: u8) -> Result<u8, ThermalPolicyError> {
    match level {
        0..=3 => Ok(level),
        _ => Err(ThermalPolicyError::LevelOutOfRange { level }),
    }
}

/// A single class-0x0d thermal command payload, independent of HID transport.
/// The command class is always 0x0d; only the id, the declared request data
/// size, and the 80 argument bytes vary. This is data, not behavior: the
/// builders below fill it and the device layer frames it into a HID report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThermalCommand {
    pub command_id: u8,
    pub data_size: u8,
    pub args: [u8; 80],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalDecodeError {
    UnexpectedFanCount { expected: u8, actual: u8 },
    UnknownFanId { id: u8, index: usize },
    DuplicateFanId { id: u8 },
    MissingFanId { fan: FanId },
    UnexpectedProfile { expected: u8, actual: u8 },
    UnexpectedFan { expected: u8, actual: u8 },
}

/// Profile slot every per-fan thermal command targets. Pinning this to 1 keeps
/// reads and writes on the same EC slot; earlier builds wrote boost commands to
/// profile 0, an inactive slot.
const THERMAL_PROFILE: u8 = 1;

/// Build the 0x80 Get Thermal Fan ID List request. Request data size 80 mirrors
/// Synapse's variable-length list read (request tuple `[80, 13, 128]`).
pub fn get_fan_ids() -> ThermalCommand {
    ThermalCommand {
        command_id: 0x80,
        data_size: 80,
        args: [0u8; 80],
    }
}

/// Build the 0x01 Set Thermal Fan Speed request: `[profile=1, fan_id, rpm/100]`.
pub fn set_fan_speed(fan: FanId, rpm: FanRpm) -> ThermalCommand {
    let mut args: [u8; 80] = [0; 80];
    args[0] = THERMAL_PROFILE;
    args[1] = fan.wire_value();
    args[2] = (rpm.0 / 100) as u8;
    ThermalCommand {
        command_id: 0x01,
        data_size: 3,
        args,
    }
}

/// Build the 0x02 Set Thermal Fan Mode request:
/// `[profile=1, fan_id, performance_mode, manual_flag]`.
pub fn set_fan_mode(fan: FanId, mode: PerformanceMode, manual: bool) -> ThermalCommand {
    let mut args: [u8; 80] = [0; 80];
    args[0] = THERMAL_PROFILE;
    args[1] = fan.wire_value();
    args[2] = mode.wire_value();
    args[3] = u8::from(manual);
    ThermalCommand {
        command_id: 0x02,
        data_size: 4,
        args,
    }
}

/// Build the 0x07 Set Custom CPU/GPU Level request: `[profile=1, fan_id, level]`.
pub fn set_boost(fan: FanId, level: u8) -> ThermalCommand {
    let mut args: [u8; 80] = [0; 80];
    args[0] = THERMAL_PROFILE;
    args[1] = fan.wire_value();
    args[2] = level;
    ThermalCommand {
        command_id: 0x07,
        data_size: 3,
        args,
    }
}

/// Build the 0x81 Get Thermal Fan Speed request: `[profile=1, fan_id]`.
pub fn get_fan_speed(fan: FanId) -> ThermalCommand {
    let mut args: [u8; 80] = [0; 80];
    args[0] = THERMAL_PROFILE;
    args[1] = fan.wire_value();
    ThermalCommand {
        command_id: 0x81,
        data_size: 3,
        args,
    }
}

/// Build the 0x82 Get Thermal Fan Mode request: `[profile=1, fan_id]`.
pub fn get_fan_mode(fan: FanId) -> ThermalCommand {
    let mut args: [u8; 80] = [0; 80];
    args[0] = THERMAL_PROFILE;
    args[1] = fan.wire_value();
    ThermalCommand {
        command_id: 0x82,
        data_size: 4,
        args,
    }
}

/// Build the 0x87 Get Custom CPU/GPU Level request: `[profile=1, fan_id]`.
pub fn get_boost(fan: FanId) -> ThermalCommand {
    let mut args: [u8; 80] = [0; 80];
    args[0] = THERMAL_PROFILE;
    args[1] = fan.wire_value();
    ThermalCommand {
        command_id: 0x87,
        data_size: 3,
        args,
    }
}

/// Build the 0x88 Get Thermal Fan Current Speed request: `[profile=1, fan_id]`.
pub fn get_current_fan_rpm(fan: FanId) -> ThermalCommand {
    let mut args: [u8; 80] = [0; 80];
    args[0] = THERMAL_PROFILE;
    args[1] = fan.wire_value();
    ThermalCommand {
        command_id: 0x88,
        data_size: 3,
        args,
    }
}

/// Decode the 0x80 fan-ID list reply. The payload is count-prefixed: `args[0]`
/// is the count, `args[1..1 + count]` are the IDs, and any following bytes are
/// padding. The Blade 16 2025 preflight requires exactly the set `{CPU, GPU}`;
/// order is normalized to `[Cpu, Gpu]` on success.
pub fn decode_fan_ids(args: &[u8; 80]) -> Result<[FanId; 2], ThermalDecodeError> {
    const EXPECTED_COUNT: u8 = 2;
    let count: u8 = args[0];
    if count != EXPECTED_COUNT {
        return Err(ThermalDecodeError::UnexpectedFanCount {
            expected: EXPECTED_COUNT,
            actual: count,
        });
    }
    // count == 2 is verified before any indexing, so args[1] and args[2] are in bounds.
    let first: FanId = decode_fan_id_byte(args[1], 1)?;
    let second: FanId = decode_fan_id_byte(args[2], 2)?;
    if first == second {
        return Err(ThermalDecodeError::DuplicateFanId { id: args[1] });
    }
    if first != FanId::Cpu && second != FanId::Cpu {
        return Err(ThermalDecodeError::MissingFanId { fan: FanId::Cpu });
    }
    if first != FanId::Gpu && second != FanId::Gpu {
        return Err(ThermalDecodeError::MissingFanId { fan: FanId::Gpu });
    }
    Ok([FanId::Cpu, FanId::Gpu])
}

fn decode_fan_id_byte(raw: u8, index: usize) -> Result<FanId, ThermalDecodeError> {
    FanId::from_wire(raw).ok_or(ThermalDecodeError::UnknownFanId { id: raw, index })
}

/// Decode a 0x88 Get Thermal Fan Current Speed reply: `[profile=1, fan_id,
/// rpm/100]`. Rejects a reply whose profile or fan byte does not match the
/// request so a stale reply is never read as the requested zone.
pub fn decode_fan_rpm(fan: FanId, args: &[u8; 80]) -> Result<u16, ThermalDecodeError> {
    verify_reply_identity(fan, args)?;
    Ok(u16::from(args[2]) * 100)
}

/// Decode a 0x82 Get Thermal Fan Mode reply: `[profile=1, fan_id, mode,
/// manual_flag]`. Rejects a reply whose profile or fan byte does not match the
/// request so a stale reply is never trusted as this zone's mode.
pub fn decode_fan_mode(fan: FanId, args: &[u8; 80]) -> Result<(u8, u8), ThermalDecodeError> {
    verify_reply_identity(fan, args)?;
    Ok((args[2], args[3]))
}

/// Decode a 0x87 Get Custom CPU/GPU Level reply: `[profile=1, fan_id, level]`.
/// Rejects a reply whose profile or fan byte does not match the request.
pub fn decode_boost(fan: FanId, args: &[u8; 80]) -> Result<u8, ThermalDecodeError> {
    verify_reply_identity(fan, args)?;
    Ok(args[2])
}

/// Decode a 0x81 Get Thermal Fan Speed reply: `[profile=1, fan_id, rpm/100]`,
/// returning the stored setpoint in RPM. Rejects a reply whose profile or fan
/// byte does not match the request.
pub fn decode_fan_setpoint(fan: FanId, args: &[u8; 80]) -> Result<u16, ThermalDecodeError> {
    verify_reply_identity(fan, args)?;
    Ok(u16::from(args[2]) * 100)
}

/// Reject a per-fan reply whose profile byte or fan byte does not match the
/// request, so a stale or cross-zone reply is never read as this zone's value.
fn verify_reply_identity(fan: FanId, args: &[u8; 80]) -> Result<(), ThermalDecodeError> {
    if args[0] != THERMAL_PROFILE {
        return Err(ThermalDecodeError::UnexpectedProfile {
            expected: THERMAL_PROFILE,
            actual: args[0],
        });
    }
    let expected_fan: u8 = fan.wire_value();
    if args[1] != expected_fan {
        return Err(ThermalDecodeError::UnexpectedFan {
            expected: expected_fan,
            actual: args[1],
        });
    }
    Ok(())
}

/// The getter-only diagnostic sweep the daemon runs before it trusts the EC.
/// It is structurally unable to mutate EC state: it calls only the `get_*`
/// builders, so it can never emit 0x01 (Set Fan Speed), 0x02 (Set Fan Mode), or
/// 0x07 (Set Custom Level). It queries the fan-ID list once, then Get Fan Speed
/// (0x81), Get Fan Mode (0x82), Get Custom Level (0x87), and Get Current RPM
/// (0x88) for both zones.
pub fn preflight_plan() -> Vec<ThermalCommand> {
    let mut plan: Vec<ThermalCommand> = Vec::with_capacity(9);
    plan.push(get_fan_ids());
    for fan in [FanId::Cpu, FanId::Gpu] {
        plan.push(get_fan_speed(fan));
        plan.push(get_fan_mode(fan));
        plan.push(get_boost(fan));
        plan.push(get_current_fan_rpm(fan));
    }
    plan
}

/// One zone's decoded current tachometer reading, gathered during preflight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneTelemetry {
    pub fan: FanId,
    pub current_rpm: u16,
}

/// The typed result of a successful preflight sweep: the confirmed fan set and
/// each zone's decoded current RPM. Absence of this report (an error instead)
/// is what drives the daemon into its Disabled safety state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightReport {
    pub fans: [FanId; 2],
    pub zones: Vec<ZoneTelemetry>,
}

/// The daemon's live thermal-safety posture.
///
/// - `Preflight`: initial posture before the getter-only sweep has decided.
/// - `Ready`: the EC answered preflight and no fixed fan speed is being watched.
/// - `Manual`: a fixed fan speed is applied and its tachometer is being verified
///   each monitoring cycle; `consecutive_failures` counts back-to-back failed
///   cycles and resets to zero on any passing cycle.
/// - `Disabled`: preflight failed, or failback fired after two consecutive
///   failures; every power/fan write is refused before command construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalSafetyState {
    Preflight,
    Ready,
    Manual {
        target: FanRpm,
        consecutive_failures: u8,
    },
    Disabled,
}

impl ThermalSafetyState {
    /// Whether the daemon must refuse every power/fan write. True only in
    /// `Disabled`, the terminal posture entered after failback or a failed
    /// preflight.
    pub fn writes_disabled(self) -> bool {
        matches!(self, ThermalSafetyState::Disabled)
    }
}

/// Two consecutive failed verification cycles trip failback. One failure is a
/// transient the next passing cycle clears; the second in a row is treated as a
/// real loss of fan control.
const FAILBACK_FAILURE_THRESHOLD: u8 = 2;

/// The outcome of one verification cycle, fed to `advance_safety`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationEvent {
    Succeeded,
    Failed(ThermalFailure),
}

/// A verification failure classified for the pure safety state machine. The
/// device layer logs the full typed transport/decode error and maps it to one
/// of these classes; `advance_safety` only needs the class, not the detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalFailure {
    /// A command did not complete over the HID transport, or a reply could not
    /// be decoded for its own zone.
    Transport,
    /// A readback could not be validated for its own zone (a stale or
    /// cross-zone reply). The full typed decode error is logged at the edge; the
    /// state machine only needs the class.
    ReadbackMismatch,
    /// A tachometer sample read back as zero while a fixed speed was commanded.
    TelemetryZero,
    /// A tachometer sample was still outside tolerance after the convergence
    /// deadline that bounds the EC's slew-limited ramp.
    ConvergenceTimeout {
        target: u16,
        observed: u16,
        tolerance: u16,
    },
}

/// The corrective action a transition requests. Failback returns both fan zones
/// to firmware-automatic control; it is the only action the machine emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyAction {
    FailbackBothFans,
}

/// The next state plus any corrective action the caller must perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyTransition {
    pub state: ThermalSafetyState,
    pub action: Option<SafetyAction>,
}

/// Advance the safety state machine for one verification cycle. Pure: it decides
/// the next state and whether failback must run, and performs no I/O.
///
/// Only the `Manual` posture is monitored. A passing cycle clears the failure
/// counter; a failing cycle increments it, and the second consecutive failure
/// transitions to `Disabled` while requesting `FailbackBothFans`. Every other
/// posture is a fixed point: the state is returned unchanged with no action.
pub fn advance_safety(state: ThermalSafetyState, event: VerificationEvent) -> SafetyTransition {
    let (target, consecutive_failures) = match state {
        ThermalSafetyState::Manual {
            target,
            consecutive_failures,
        } => (target, consecutive_failures),
        other => {
            return SafetyTransition {
                state: other,
                action: None,
            };
        }
    };
    match event {
        VerificationEvent::Succeeded => SafetyTransition {
            state: ThermalSafetyState::Manual {
                target,
                consecutive_failures: 0,
            },
            action: None,
        },
        VerificationEvent::Failed(_) => {
            let failures: u8 = consecutive_failures.saturating_add(1);
            if failures >= FAILBACK_FAILURE_THRESHOLD {
                SafetyTransition {
                    state: ThermalSafetyState::Disabled,
                    action: Some(SafetyAction::FailbackBothFans),
                }
            } else {
                SafetyTransition {
                    state: ThermalSafetyState::Manual {
                        target,
                        consecutive_failures: failures,
                    },
                    action: None,
                }
            }
        }
    }
}

/// Classify one fixed-mode tachometer reading against its commanded target.
///
/// A zero sample (fan stopped) always fails. A sample inside
/// `manual_tolerance` passes. Outside tolerance, the verdict depends on the
/// caller's convergence deadline: the EC slews manual targets at a measured
/// ~35-40 RPM/s with 100-RPM tach quantization and multi-second pauses, so an
/// out-of-tolerance sample is a healthy ramp until the deadline and a failure
/// after it. Pure so the monitoring decision is unit-tested independent of
/// HID I/O.
pub fn classify_manual_reading(
    target: FanRpm,
    observed: u16,
    ramp_deadline_exceeded: bool,
) -> VerificationEvent {
    if observed == 0 {
        return VerificationEvent::Failed(ThermalFailure::TelemetryZero);
    }
    let tolerance: u16 = manual_tolerance(target.0);
    if observed.abs_diff(target.0) <= tolerance {
        return VerificationEvent::Succeeded;
    }
    if !ramp_deadline_exceeded {
        return VerificationEvent::Succeeded;
    }
    VerificationEvent::Failed(ThermalFailure::ConvergenceTimeout {
        target: target.0,
        observed,
        tolerance,
    })
}

/// The monitor's identity for the armed fixed target: its value plus the apply
/// generation that last armed it. The generation changes on every fresh apply,
/// even a same-value re-apply (e.g. a wake re-latch), so the monitor can restart
/// the settle window instead of letting two spin-up-time ticks trip failback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManualWatch {
    pub target: FanRpm,
    pub generation: u64,
}

/// One monitor tick's pure decision: run a verification cycle, keep waiting, or
/// restart the settle window for a freshly observed (or changed) target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitorDecision {
    RestartSettle,
    Waiting,
    RunCycle,
}

/// Decide one monitor tick. Pure: the settle window (whose timing the caller
/// owns) restarts whenever the observed watch differs from the tracked one, so a
/// changed target value *or* a fresh apply generation re-arms it before any
/// tachometer check. Only when the same identity is still tracked and its window
/// has elapsed is a verification cycle due.
pub fn decide_monitor_tick(
    tracked: Option<ManualWatch>,
    observed: ManualWatch,
    settle_elapsed: bool,
) -> MonitorDecision {
    if tracked != Some(observed) {
        return MonitorDecision::RestartSettle;
    }
    if settle_elapsed {
        MonitorDecision::RunCycle
    } else {
        MonitorDecision::Waiting
    }
}

/// The outcome of returning one fan zone to firmware-automatic control during
/// failback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneOutcome {
    Restored,
    Failed,
}

/// The result of a two-zone failback. Both zones are always attempted, so both
/// outcomes are retained even when the first zone fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FailbackReport {
    pub cpu: ZoneOutcome,
    pub gpu: ZoneOutcome,
}

/// Seconds after the wake signal at which the single getter-only re-verification
/// runs. The firmware may finish resetting the GPU power zone several seconds
/// into resume, and UPower can be slow to settle the AC state, so the readback
/// waits for both before deciding whether one repair is warranted.
pub const WAKE_DELAYED_VERIFY_SECS: u64 = 10;

/// One step of the burst-free post-wake re-latch sequence (Blade 16 2025).
/// `delay_secs` is measured from the wake signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WakeStep {
    pub delay_secs: u64,
    pub kind: WakeStepKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeStepKind {
    /// The full verified apply (write + readback), run once immediately on wake.
    ApplyAndVerify,
    /// A getter-only re-verification (no writes) that authorizes at most one
    /// repair; see `decide_wake_repair`.
    DelayedReadback,
}

/// The post-wake re-latch sequence: exactly one immediate verified apply, then
/// one getter-only readback `WAKE_DELAYED_VERIFY_SECS` later. Deliberately
/// burst-free: the delayed readback, not repetition, decides whether the single
/// repair runs.
pub fn wake_sequence() -> [WakeStep; 2] {
    [
        WakeStep {
            delay_secs: 0,
            kind: WakeStepKind::ApplyAndVerify,
        },
        WakeStep {
            delay_secs: WAKE_DELAYED_VERIFY_SECS,
            kind: WakeStepKind::DelayedReadback,
        },
    ]
}

/// The outcome of the delayed getter-only readback: whether the applied thermal
/// state still holds, or the firmware reset it during resume.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeVerifyOutcome {
    StateHeld,
    StateReset,
}

/// The corrective step after the delayed readback. At most one repair is ever
/// issued, so there is no multi-repair variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeRepair {
    None,
    ReapplyOnce,
}

/// The delayed readback authorizes at most one repair, and only when it found the
/// applied state reset. A held state needs no write.
pub fn decide_wake_repair(outcome: WakeVerifyOutcome) -> WakeRepair {
    match outcome {
        WakeVerifyOutcome::StateHeld => WakeRepair::None,
        WakeVerifyOutcome::StateReset => WakeRepair::ReapplyOnce,
    }
}

/// Fold the single post-wake repair's result into the shared failure counter, so
/// a failed repair advances the same state machine a failed monitoring cycle
/// would (the second consecutive failure trips failback) rather than a bespoke
/// path. A successful repair clears the counter like a passing cycle.
pub fn advance_after_repair(
    state: ThermalSafetyState,
    repair: Result<(), ThermalFailure>,
) -> SafetyTransition {
    let event: VerificationEvent = match repair {
        Ok(()) => VerificationEvent::Succeeded,
        Err(failure) => VerificationEvent::Failed(failure),
    };
    advance_safety(state, event)
}

/// The dGPU inactive-to-active relatch commands for the Blade 16 2025. In Custom
/// mode the daemon re-latches only the GPU custom level: write it (0x07) then read
/// it back (0x87), touching no CPU state and no fan state. Outside Custom there is
/// nothing mode-specific to re-latch, so the plan is empty. The level is passed
/// through unchanged; the caller validates it before any write.
pub fn dgpu_resume_plan(mode: PerformanceMode, gpu_level: u8) -> Vec<ThermalCommand> {
    if mode != PerformanceMode::Custom {
        return Vec::new();
    }
    vec![set_boost(FanId::Gpu, gpu_level), get_boost(FanId::Gpu)]
}

/// How the daemon binary was invoked. The modes are mutually exclusive and
/// matched once at startup, so no per-mode behavior flag is threaded downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonExecution {
    Service,
    PreflightOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionParseError {
    UnknownFlag(String),
}

impl std::fmt::Display for ExecutionParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionParseError::UnknownFlag(flag) => write!(formatter, "unknown flag {flag}"),
        }
    }
}

const PREFLIGHT_ONLY_FLAG: &str = "--preflight-only";

/// Parse the daemon's execution mode from its argument flags (argv without the
/// program name). Any unrecognized flag is a hard parse error; a repeated
/// identical flag is harmless.
pub fn parse_execution(flags: &[String]) -> Result<DaemonExecution, ExecutionParseError> {
    let mut selected: Option<DaemonExecution> = None;
    for flag in flags {
        let requested = match flag.as_str() {
            PREFLIGHT_ONLY_FLAG => DaemonExecution::PreflightOnly,
            other => return Err(ExecutionParseError::UnknownFlag(other.to_string())),
        };
        selected = Some(requested);
    }
    Ok(selected.unwrap_or(DaemonExecution::Service))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_only_valid_modes() {
        assert_eq!(
            selectable_modes(PowerSource::Ac),
            &[
                PerformanceMode::Balanced,
                PerformanceMode::Silent,
                PerformanceMode::MaximumPerformance,
                PerformanceMode::Custom,
                PerformanceMode::Hyperboost,
            ]
        );
        assert_eq!(
            selectable_modes(PowerSource::Battery),
            &[
                PerformanceMode::BalancedBattery,
                PerformanceMode::BatterySaver
            ]
        );
    }

    #[test]
    fn validates_fixed_rpm_without_clamping() {
        assert_eq!(validate_fixed_rpm(PerformanceMode::Balanced, 0), Ok(None));
        assert_eq!(
            validate_fixed_rpm(PerformanceMode::Balanced, 3400),
            Ok(Some(FanRpm(3400)))
        );
        assert!(matches!(
            validate_fixed_rpm(PerformanceMode::Balanced, 3399),
            Err(ThermalPolicyError::RpmOutOfRange { .. })
        ));
        assert!(matches!(
            validate_fixed_rpm(PerformanceMode::Balanced, 5201),
            Err(ThermalPolicyError::RpmOutOfRange { .. })
        ));
    }

    #[test]
    fn exposes_hardware_validated_features() {
        // Custom level 3 (Extreme) and Hyperboost (7) latch-validated on the
        // physical unit 2026-07-13: written, read back identically, restored.
        // Hyperboost's runtime power behavior is owner-tested; Razer pairs it
        // with the cooling pad, so the UIs label it accordingly. AC-only.
        assert!(is_mode_selectable(
            PowerSource::Ac,
            PerformanceMode::Hyperboost
        ));
        assert!(!is_mode_selectable(
            PowerSource::Battery,
            PerformanceMode::Hyperboost
        ));
        assert_eq!(validate_custom_level(3), Ok(3));
    }

    #[test]
    fn decodes_recognized_wire_values_only() {
        assert_eq!(
            PerformanceMode::try_from(0u8),
            Ok(PerformanceMode::Balanced)
        );
        assert_eq!(
            PerformanceMode::try_from(2u8),
            Ok(PerformanceMode::MaximumPerformance)
        );
        assert_eq!(
            PerformanceMode::try_from(3u8),
            Ok(PerformanceMode::BatterySaver)
        );
        assert_eq!(PerformanceMode::try_from(4u8), Ok(PerformanceMode::Custom));
        assert_eq!(PerformanceMode::try_from(5u8), Ok(PerformanceMode::Silent));
        assert_eq!(
            PerformanceMode::try_from(7u8),
            Ok(PerformanceMode::Hyperboost)
        );
        assert_eq!(
            PerformanceMode::try_from(6u8),
            Ok(PerformanceMode::BalancedBattery)
        );
        assert_eq!(
            PerformanceMode::try_from(1u8),
            Err(ThermalPolicyError::UnknownMode { wire_value: 1 })
        );
    }

    #[test]
    fn wire_values_round_trip() {
        for mode in [
            PerformanceMode::Balanced,
            PerformanceMode::BalancedBattery,
            PerformanceMode::MaximumPerformance,
            PerformanceMode::BatterySaver,
            PerformanceMode::Custom,
            PerformanceMode::Silent,
            PerformanceMode::Hyperboost,
        ] {
            assert_eq!(PerformanceMode::try_from(mode.wire_value()), Ok(mode));
        }
    }

    #[test]
    fn fan_ids_match_ec_zones() {
        assert_eq!(FanId::Cpu.wire_value(), 1);
        assert_eq!(FanId::Gpu.wire_value(), 2);
    }

    #[test]
    fn rejects_unavailable_mode_for_power_source() {
        // Balanced is domain-partitioned: wire 0 is the AC slot, wire 6 the
        // battery slot, and neither is selectable in the other domain.
        assert!(!is_mode_selectable(
            PowerSource::Battery,
            PerformanceMode::Balanced
        ));
        assert!(!is_mode_selectable(
            PowerSource::Ac,
            PerformanceMode::BalancedBattery
        ));
        assert!(!is_mode_selectable(
            PowerSource::Battery,
            PerformanceMode::MaximumPerformance
        ));
        assert!(!is_mode_selectable(
            PowerSource::Battery,
            PerformanceMode::Silent
        ));
        assert!(!is_mode_selectable(
            PowerSource::Battery,
            PerformanceMode::Custom
        ));
        assert!(!is_mode_selectable(
            PowerSource::Ac,
            PerformanceMode::BatterySaver
        ));
        assert!(!is_mode_selectable(
            PowerSource::Battery,
            PerformanceMode::Hyperboost
        ));
        assert!(is_mode_selectable(
            PowerSource::Ac,
            PerformanceMode::MaximumPerformance
        ));
    }

    #[test]
    fn validates_custom_levels() {
        assert_eq!(validate_custom_level(0), Ok(0));
        assert_eq!(validate_custom_level(1), Ok(1));
        assert_eq!(validate_custom_level(2), Ok(2));
        assert_eq!(
            validate_custom_level(4),
            Err(ThermalPolicyError::LevelOutOfRange { level: 4 })
        );
    }

    #[test]
    fn rpm_boundaries_for_every_mode() {
        let cases: [(PerformanceMode, u16, u16); 6] = [
            (PerformanceMode::Balanced, 3400, 5200),
            (PerformanceMode::Silent, 3400, 5200),
            (PerformanceMode::MaximumPerformance, 3300, 5400),
            (PerformanceMode::BatterySaver, 3300, 5400),
            (PerformanceMode::Custom, 4000, 5300),
            (PerformanceMode::Hyperboost, 3700, 5300),
        ];
        for (mode, min, max) in cases {
            assert_eq!(provisional_rpm_range(mode), RpmRange { min, max });
            assert!(matches!(
                validate_fixed_rpm(mode, i32::from(min) - 1),
                Err(ThermalPolicyError::RpmOutOfRange { .. })
            ));
            assert_eq!(
                validate_fixed_rpm(mode, i32::from(min)),
                Ok(Some(FanRpm(min)))
            );
            assert_eq!(
                validate_fixed_rpm(mode, i32::from(max)),
                Ok(Some(FanRpm(max)))
            );
            assert!(matches!(
                validate_fixed_rpm(mode, i32::from(max) + 1),
                Err(ThermalPolicyError::RpmOutOfRange { .. })
            ));
            assert_eq!(validate_fixed_rpm(mode, 0), Ok(None));
        }
    }

    #[test]
    fn tolerance_scales_with_target() {
        assert_eq!(manual_tolerance(3400), 850);
        assert_eq!(manual_tolerance(4000), 1000);
        assert_eq!(manual_tolerance(5400), 1350);
        assert_eq!(manual_tolerance(1000), 500);
    }

    #[test]
    fn failure_then_success_resets_the_counter() {
        let initial: ThermalSafetyState = ThermalSafetyState::Manual {
            target: FanRpm(4000),
            consecutive_failures: 0,
        };
        let failed: SafetyTransition = advance_safety(
            initial,
            VerificationEvent::Failed(ThermalFailure::TelemetryZero),
        );
        assert_eq!(
            failed.state,
            ThermalSafetyState::Manual {
                target: FanRpm(4000),
                consecutive_failures: 1
            }
        );
        assert_eq!(failed.action, None);
        let recovered: SafetyTransition =
            advance_safety(failed.state, VerificationEvent::Succeeded);
        assert_eq!(
            recovered.state,
            ThermalSafetyState::Manual {
                target: FanRpm(4000),
                consecutive_failures: 0
            }
        );
        assert_eq!(recovered.action, None);
    }

    #[test]
    fn two_consecutive_failures_request_failback_and_disable() {
        let initial: ThermalSafetyState = ThermalSafetyState::Manual {
            target: FanRpm(4000),
            consecutive_failures: 0,
        };
        let first: SafetyTransition = advance_safety(
            initial,
            VerificationEvent::Failed(ThermalFailure::TelemetryZero),
        );
        let second: SafetyTransition = advance_safety(
            first.state,
            VerificationEvent::Failed(ThermalFailure::TelemetryZero),
        );
        assert_eq!(second.action, Some(SafetyAction::FailbackBothFans));
        assert_eq!(second.state, ThermalSafetyState::Disabled);
    }

    #[test]
    fn disabled_rejects_every_later_write() {
        assert!(ThermalSafetyState::Disabled.writes_disabled());
        assert!(!ThermalSafetyState::Ready.writes_disabled());
        assert!(!ThermalSafetyState::Preflight.writes_disabled());
        assert!(
            !ThermalSafetyState::Manual {
                target: FanRpm(4000),
                consecutive_failures: 1
            }
            .writes_disabled()
        );
        // Disabled is terminal: any further event keeps it Disabled with no action.
        let after: SafetyTransition = advance_safety(
            ThermalSafetyState::Disabled,
            VerificationEvent::Failed(ThermalFailure::Transport),
        );
        assert_eq!(after.state, ThermalSafetyState::Disabled);
        assert_eq!(after.action, None);
    }

    #[test]
    fn failback_report_retains_both_results_when_cpu_fails() {
        let report: FailbackReport = FailbackReport {
            cpu: ZoneOutcome::Failed,
            gpu: ZoneOutcome::Restored,
        };
        // The GPU zone is attempted even though the CPU zone failed, so both
        // outcomes are retained.
        assert_eq!(report.cpu, ZoneOutcome::Failed);
        assert_eq!(report.gpu, ZoneOutcome::Restored);
    }

    #[test]
    fn classifies_manual_reading_within_and_beyond_tolerance() {
        let cases: [(u16, u16); 3] = [(3400, 850), (4000, 1000), (5400, 1350)];
        for (target, tolerance) in cases {
            assert_eq!(manual_tolerance(target), tolerance);
            for deadline_exceeded in [false, true] {
                assert_eq!(
                    classify_manual_reading(FanRpm(target), target, deadline_exceeded),
                    VerificationEvent::Succeeded
                );
                assert_eq!(
                    classify_manual_reading(FanRpm(target), target + tolerance, deadline_exceeded),
                    VerificationEvent::Succeeded
                );
                assert_eq!(
                    classify_manual_reading(FanRpm(target), target - tolerance, deadline_exceeded),
                    VerificationEvent::Succeeded
                );
                // A stopped fan fails immediately, ramp window or not.
                assert_eq!(
                    classify_manual_reading(FanRpm(target), 0, deadline_exceeded),
                    VerificationEvent::Failed(ThermalFailure::TelemetryZero)
                );
            }
        }
    }

    #[test]
    fn out_of_tolerance_sample_is_benign_while_ramping() {
        // Measured on the validation unit: the EC slews manual targets at
        // ~35-40 RPM/s with 100-RPM tach quantization and multi-second pauses,
        // so per-cycle progress cannot be judged. Any spinning out-of-tolerance
        // sample is healthy until the convergence deadline.
        assert_eq!(
            classify_manual_reading(FanRpm(4000), 2300, false),
            VerificationEvent::Succeeded
        );
    }

    #[test]
    fn out_of_tolerance_sample_fails_after_the_convergence_deadline() {
        assert_eq!(
            classify_manual_reading(FanRpm(4000), 2300, true),
            VerificationEvent::Failed(ThermalFailure::ConvergenceTimeout {
                target: 4000,
                observed: 2300,
                tolerance: 1000,
            })
        );
    }

    #[test]
    fn fresh_apply_generation_restarts_settle_window() {
        let watch_v1: ManualWatch = ManualWatch {
            target: FanRpm(4000),
            generation: 1,
        };
        // Same target and generation, settle window elapsed: a cycle is due.
        assert_eq!(
            decide_monitor_tick(Some(watch_v1), watch_v1, true),
            MonitorDecision::RunCycle
        );
        // Same identity, still inside the window: wait.
        assert_eq!(
            decide_monitor_tick(Some(watch_v1), watch_v1, false),
            MonitorDecision::Waiting
        );
        // A fresh apply of the SAME target value bumps the generation, so the
        // settle window restarts even though the window had elapsed. Two spin-up
        // ticks after a wake re-apply must not be able to trip failback.
        let watch_v2: ManualWatch = ManualWatch {
            target: FanRpm(4000),
            generation: 2,
        };
        assert_eq!(
            decide_monitor_tick(Some(watch_v1), watch_v2, true),
            MonitorDecision::RestartSettle
        );
        // A changed target value also restarts the window.
        let watch_changed: ManualWatch = ManualWatch {
            target: FanRpm(5000),
            generation: 1,
        };
        assert_eq!(
            decide_monitor_tick(Some(watch_v1), watch_changed, true),
            MonitorDecision::RestartSettle
        );
        // The first observation with nothing tracked starts the window.
        assert_eq!(
            decide_monitor_tick(None, watch_v1, true),
            MonitorDecision::RestartSettle
        );
    }

    #[test]
    fn decodes_fan_mode_with_identity_check() {
        let mut args: [u8; 80] = [0; 80];
        args[..4].copy_from_slice(&[1, 1, 4, 1]);
        assert_eq!(decode_fan_mode(FanId::Cpu, &args), Ok((4, 1)));
        assert_eq!(
            decode_fan_mode(FanId::Gpu, &args),
            Err(ThermalDecodeError::UnexpectedFan {
                expected: 2,
                actual: 1
            })
        );
        args[0] = 0;
        assert_eq!(
            decode_fan_mode(FanId::Cpu, &args),
            Err(ThermalDecodeError::UnexpectedProfile {
                expected: 1,
                actual: 0
            })
        );
    }

    #[test]
    fn decodes_boost_with_identity_check() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[1, 2, 2]);
        assert_eq!(decode_boost(FanId::Gpu, &args), Ok(2));
        assert_eq!(
            decode_boost(FanId::Cpu, &args),
            Err(ThermalDecodeError::UnexpectedFan {
                expected: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn decodes_fan_setpoint_with_identity_check() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[1, 1, 34]);
        assert_eq!(decode_fan_setpoint(FanId::Cpu, &args), Ok(3400));
        args[0] = 2;
        assert_eq!(
            decode_fan_setpoint(FanId::Cpu, &args),
            Err(ThermalDecodeError::UnexpectedProfile {
                expected: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn builds_get_fan_ids_request() {
        let command: ThermalCommand = get_fan_ids();
        assert_eq!(command.command_id, 0x80);
        assert_eq!(command.data_size, 80);
        assert_eq!(command.args, [0u8; 80]);
    }

    #[test]
    fn builds_set_fan_speed_request() {
        let command: ThermalCommand = set_fan_speed(FanId::Cpu, FanRpm(3400));
        assert_eq!(command.command_id, 0x01);
        assert_eq!(command.data_size, 3);
        assert_eq!(&command.args[..3], &[1, 1, 34]);
    }

    #[test]
    fn builds_set_fan_mode_request() {
        let command: ThermalCommand =
            set_fan_mode(FanId::Gpu, PerformanceMode::MaximumPerformance, true);
        assert_eq!(command.command_id, 0x02);
        assert_eq!(command.data_size, 4);
        assert_eq!(&command.args[..4], &[1, 2, 2, 1]);
    }

    #[test]
    fn builds_set_boost_request_with_profile_one() {
        // Regression guard: earlier builds shipped boost with profile byte 0.
        let command: ThermalCommand = set_boost(FanId::Cpu, 2);
        assert_eq!(command.command_id, 0x07);
        assert_eq!(command.data_size, 3);
        assert_eq!(&command.args[..3], &[1, 1, 2]);
    }

    #[test]
    fn builds_get_fan_speed_request() {
        let command: ThermalCommand = get_fan_speed(FanId::Cpu);
        assert_eq!(command.command_id, 0x81);
        assert_eq!(command.data_size, 3);
        assert_eq!(&command.args[..2], &[1, 1]);
    }

    #[test]
    fn builds_get_fan_mode_request() {
        let command: ThermalCommand = get_fan_mode(FanId::Gpu);
        assert_eq!(command.command_id, 0x82);
        assert_eq!(command.data_size, 4);
        assert_eq!(&command.args[..2], &[1, 2]);
    }

    #[test]
    fn builds_get_boost_request_with_profile_one() {
        let command: ThermalCommand = get_boost(FanId::Gpu);
        assert_eq!(command.command_id, 0x87);
        assert_eq!(command.data_size, 3);
        assert_eq!(&command.args[..2], &[1, 2]);
    }

    #[test]
    fn builds_get_current_fan_rpm_request() {
        let command: ThermalCommand = get_current_fan_rpm(FanId::Cpu);
        assert_eq!(command.command_id, 0x88);
        assert_eq!(command.data_size, 3);
        assert_eq!(&command.args[..2], &[1, 1]);
    }

    #[test]
    fn decodes_current_fan_rpm() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[1, 2, 34]);
        assert_eq!(decode_fan_rpm(FanId::Gpu, &args), Ok(3400));
    }

    #[test]
    fn rejects_fan_rpm_with_wrong_profile() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[0, 1, 34]);
        assert_eq!(
            decode_fan_rpm(FanId::Cpu, &args),
            Err(ThermalDecodeError::UnexpectedProfile {
                expected: 1,
                actual: 0
            })
        );
    }

    #[test]
    fn rejects_fan_rpm_with_wrong_fan() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[1, 2, 34]);
        assert_eq!(
            decode_fan_rpm(FanId::Cpu, &args),
            Err(ThermalDecodeError::UnexpectedFan {
                expected: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn decodes_count_prefixed_fan_ids() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[2, 1, 2]);
        assert_eq!(decode_fan_ids(&args), Ok([FanId::Cpu, FanId::Gpu]));
    }

    #[test]
    fn normalizes_reversed_fan_id_order() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[2, 2, 1]);
        assert_eq!(decode_fan_ids(&args), Ok([FanId::Cpu, FanId::Gpu]));
    }

    #[test]
    fn ignores_padding_after_counted_ids() {
        let mut args: [u8; 80] = [0; 80];
        args[..5].copy_from_slice(&[2, 1, 2, 9, 9]);
        assert_eq!(decode_fan_ids(&args), Ok([FanId::Cpu, FanId::Gpu]));
    }

    #[test]
    fn rejects_wrong_fan_count() {
        for count in [0_u8, 1, 3, 80] {
            let mut args: [u8; 80] = [0; 80];
            args[0] = count;
            assert!(matches!(
                decode_fan_ids(&args),
                Err(ThermalDecodeError::UnexpectedFanCount { expected: 2, actual }) if actual == count
            ));
        }
    }

    #[test]
    fn rejects_duplicate_fan_ids() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[2, 1, 1]);
        assert_eq!(
            decode_fan_ids(&args),
            Err(ThermalDecodeError::DuplicateFanId { id: 1 })
        );
    }

    #[test]
    fn rejects_unknown_fan_id() {
        let mut args: [u8; 80] = [0; 80];
        args[..3].copy_from_slice(&[2, 1, 3]);
        assert_eq!(
            decode_fan_ids(&args),
            Err(ThermalDecodeError::UnknownFanId { id: 3, index: 2 })
        );
    }

    #[test]
    fn preflight_is_getter_only() {
        let commands: Vec<ThermalCommand> = preflight_plan();
        assert!(
            commands
                .iter()
                .all(|command| matches!(command.command_id, 0x80 | 0x81 | 0x82 | 0x87 | 0x88))
        );
        assert!(
            commands
                .iter()
                .all(|command| !matches!(command.command_id, 0x01 | 0x02 | 0x07))
        );
    }

    #[test]
    fn preflight_queries_every_getter_for_both_zones() {
        let commands: Vec<ThermalCommand> = preflight_plan();
        let count = |id: u8| {
            commands
                .iter()
                .filter(|command| command.command_id == id)
                .count()
        };
        assert_eq!(count(0x80), 1);
        for per_zone_getter in [0x81_u8, 0x82, 0x87, 0x88] {
            assert_eq!(
                count(per_zone_getter),
                2,
                "getter {per_zone_getter:#x} must cover both zones"
            );
        }
    }

    #[test]
    fn parse_execution_defaults_to_service() {
        assert_eq!(parse_execution(&[]), Ok(DaemonExecution::Service));
    }

    #[test]
    fn parse_execution_selects_a_single_flag() {
        assert_eq!(
            parse_execution(&["--preflight-only".to_string()]),
            Ok(DaemonExecution::PreflightOnly)
        );
    }

    #[test]
    fn parse_execution_rejects_removed_limit_collection_flag() {
        // 0x86 Get Thermal Fan Information is NOT_SUPPORTED on the 02C6 EC
        // (verified on hardware), so the collection mode no longer exists.
        assert_eq!(
            parse_execution(&["--collect-thermal-limits".to_string()]),
            Err(ExecutionParseError::UnknownFlag(
                "--collect-thermal-limits".to_string()
            ))
        );
    }

    #[test]
    fn parse_execution_rejects_unknown_flag() {
        assert_eq!(
            parse_execution(&["--nope".to_string()]),
            Err(ExecutionParseError::UnknownFlag("--nope".to_string()))
        );
    }

    #[test]
    fn fan_id_round_trips_through_wire_value() {
        for fan in [FanId::Cpu, FanId::Gpu] {
            assert_eq!(FanId::from_wire(fan.wire_value()), Some(fan));
        }
        assert_eq!(FanId::from_wire(0), None);
        assert_eq!(FanId::from_wire(3), None);
    }

    #[test]
    fn wake_sequence_is_one_immediate_apply_then_one_delayed_readback() {
        let sequence: [WakeStep; 2] = wake_sequence();
        let immediate: Vec<&WakeStep> = sequence
            .iter()
            .filter(|step| step.kind == WakeStepKind::ApplyAndVerify)
            .collect();
        assert_eq!(immediate.len(), 1, "exactly one immediate verified apply");
        assert_eq!(
            immediate[0].delay_secs, 0,
            "the immediate apply has no delay"
        );
        let delayed: Vec<&WakeStep> = sequence
            .iter()
            .filter(|step| step.kind == WakeStepKind::DelayedReadback)
            .collect();
        assert_eq!(delayed.len(), 1, "exactly one delayed getter-only readback");
        assert_eq!(
            delayed[0].delay_secs, 10,
            "the delayed readback runs at ten seconds"
        );
        assert_eq!(delayed[0].delay_secs, WAKE_DELAYED_VERIFY_SECS);
    }

    #[test]
    fn wake_repair_runs_at_most_once_and_only_on_reset() {
        assert_eq!(
            decide_wake_repair(WakeVerifyOutcome::StateHeld),
            WakeRepair::None
        );
        assert_eq!(
            decide_wake_repair(WakeVerifyOutcome::StateReset),
            WakeRepair::ReapplyOnce
        );
    }

    #[test]
    fn failed_wake_repair_feeds_the_normal_failure_counter() {
        let armed: ThermalSafetyState = ThermalSafetyState::Manual {
            target: FanRpm(4000),
            consecutive_failures: 0,
        };
        let first: SafetyTransition = advance_after_repair(armed, Err(ThermalFailure::Transport));
        assert_eq!(
            first.state,
            ThermalSafetyState::Manual {
                target: FanRpm(4000),
                consecutive_failures: 1
            }
        );
        assert_eq!(first.action, None);
        // A second consecutive failed repair trips the same failback the monitor uses.
        let second: SafetyTransition =
            advance_after_repair(first.state, Err(ThermalFailure::Transport));
        assert_eq!(second.state, ThermalSafetyState::Disabled);
        assert_eq!(second.action, Some(SafetyAction::FailbackBothFans));
        // A successful repair clears the counter, just like a passing monitor cycle.
        let cleared: SafetyTransition = advance_after_repair(first.state, Ok(()));
        assert_eq!(
            cleared.state,
            ThermalSafetyState::Manual {
                target: FanRpm(4000),
                consecutive_failures: 0
            }
        );
        assert_eq!(cleared.action, None);
    }

    #[test]
    fn dgpu_resume_relatches_gpu_boost_only_in_custom() {
        let plan: Vec<ThermalCommand> = dgpu_resume_plan(PerformanceMode::Custom, 2);
        assert_eq!(
            plan.len(),
            2,
            "Custom relatch is exactly write then readback"
        );
        assert_eq!(plan[0].command_id, 0x07);
        assert_eq!(plan[1].command_id, 0x87);
        // Every command targets the GPU zone: no CPU state, no fan state is touched.
        assert!(
            plan.iter()
                .all(|command| command.args[1] == FanId::Gpu.wire_value())
        );
        assert!(
            plan.iter()
                .all(|command| command.args[1] != FanId::Cpu.wire_value())
        );
        assert_eq!(
            plan[0].args[2], 2,
            "the write carries the requested GPU level"
        );
    }

    #[test]
    fn dgpu_resume_issues_no_command_outside_custom() {
        for mode in [
            PerformanceMode::Balanced,
            PerformanceMode::Silent,
            PerformanceMode::MaximumPerformance,
            PerformanceMode::BatterySaver,
            PerformanceMode::Hyperboost,
        ] {
            assert!(
                dgpu_resume_plan(mode, 2).is_empty(),
                "no dGPU command outside Custom"
            );
        }
    }

    #[test]
    fn pid_gate_accepts_blade_16_2025_only() {
        assert!(is_supported_pid("02C6"));
        assert!(is_supported_pid("02c6"));
        assert!(is_supported_pid(" 02C6 "));
        assert!(!is_supported_pid("02B7"));
        assert!(!is_supported_pid("028B"));
        assert!(!is_supported_pid("not-a-pid"));
    }
}
