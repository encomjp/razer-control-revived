use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::{UnixListener, UnixStream};
use libc::umask;

/// Razer laptop control socket path
pub const SOCKET_PATH: &str = "/tmp/razercontrol-socket";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub pci_slot: String,
    pub driver: String,
    pub gpu_type: String,
    pub runtime_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents data sent TO the daemon
pub enum DaemonCommand {
    SetFanSpeed { ac: usize, rpm: i32 },      // Fan speed
    GetFanSpeed { ac: usize },                 // Get (Fan speed)
    SetPowerMode { ac: usize, pwr: u8, cpu: u8, gpu: u8}, // Power mode
    GetPwrLevel { ac: usize },                 // Get (Power mode)
    GetCPUBoost { ac: usize },                 // Get (CPU boost)
    GetGPUBoost { ac: usize },                 // Get (GPU boost)
    SetLogoLedState{ ac:usize, logo_state: u8 },
    GetLogoLedState { ac: usize },
    GetKeyboardRGB { layer: i32 }, // Layer ID
    SetEffect { name: String, params: Vec<u8> }, // Set keyboard colour
    SetStandardEffect { name: String, params: Vec<u8> }, // Set keyboard colour
    SetBrightness { ac:usize, val: u8 },
    SetIdle {ac: usize, val: u32 },
    GetBrightness { ac: usize },
    SetSync { sync: bool },
    GetSync (),
    SetBatteryHealthOptimizer { is_on: bool, threshold: u8 },
    GetBatteryHealthOptimizer (),
    GetDeviceName,
    GetActualFanRpm,
    GetStandardEffect,
    GetGpuStatus,
    SetDgpuRuntimePM { enabled: bool },
    SetGpuMode { mode: String },
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents data sent back from Daemon after it receives
/// a command.
pub enum DaemonResponse {
    SetFanSpeed { result: bool },                    // Response
    GetFanSpeed { rpm: i32 },                        // Get (Fan speed)
    SetPowerMode { result: bool },                   // Response
    GetPwrLevel { pwr: u8 },                         // Get (Power mode)
    GetCPUBoost { cpu: u8 },                         // Get (CPU boost)
    GetGPUBoost { gpu: u8 },                         // Get (GPU boost)
    SetLogoLedState {result: bool },
    GetLogoLedState { logo_state: u8 },
    GetKeyboardRGB { layer: i32, rgbdata: Vec<u8> }, // Response (RGB) of 90 keys
    SetEffect { result: bool },                       // Set keyboard colour
    SetStandardEffect { result: bool },                       // Set keyboard colour
    SetBrightness { result: bool },
    SetIdle { result: bool },
    GetBrightness { result: u8 },
    SetSync { result: bool },
    GetSync { sync: bool },
    SetBatteryHealthOptimizer { result: bool },
    GetBatteryHealthOptimizer { is_on: bool, threshold: u8 },
    GetDeviceName { name: String },
    GetActualFanRpm { rpm: i32 },
    GetStandardEffect { effect: u8, params: Vec<u8> },
    GetGpuStatus {
        gpus: Vec<GpuInfo>,
        dgpu_runtime_pm: bool,
        envycontrol_mode: String,
        envycontrol_available: bool,
    },
    SetDgpuRuntimePM { result: bool },
    SetGpuMode { result: bool, message: String },
}

#[allow(dead_code)]
pub fn bind() -> Option<UnixStream> {
    if let Ok(socket) = UnixStream::connect(SOCKET_PATH) {
        return Some(socket);
    } else {
        return None;
    }
}

#[allow(dead_code)]
/// We use this from the app, but it should replace bind
pub fn try_bind() -> std::io::Result<UnixStream> {
    UnixStream::connect(SOCKET_PATH)
}

#[allow(dead_code)]
pub fn create() -> Option<UnixListener> {
    if std::fs::metadata(SOCKET_PATH).is_ok() {
        // Socket file exists — check if a daemon is actually listening
        if UnixStream::connect(SOCKET_PATH).is_ok() {
            eprintln!("UNIX Socket already exists and a daemon is responding. Is another daemon running?");
            return None;
        }
        // Stale socket from a previous crash — remove it
        eprintln!("Removing stale socket file");
        if std::fs::remove_file(SOCKET_PATH).is_err() {
            eprintln!("Could not remove stale socket file");
            return None;
        }
    }
    // Set restrictive umask before bind so the socket is never world-accessible
    let old_umask = unsafe { umask(0o077) };
    let result = UnixListener::bind(SOCKET_PATH);
    unsafe { umask(old_umask) };
    match result {
        Ok(listener) => Some(listener),
        Err(e) => {
            eprintln!("Failed to bind socket: {}", e);
            None
        }
    }
}

#[allow(dead_code)]
pub fn send_to_daemon(command: DaemonCommand, mut sock: UnixStream) -> Option<DaemonResponse> {
    if let Ok(encoded) = bincode::serialize(&command) {
        if sock.write_all(&encoded).is_ok() {
            // Signal request EOF to daemon so it can read the full command.
            let _ = sock.shutdown(Shutdown::Write);

            let mut response = Vec::new();
            return match sock.read_to_end(&mut response) {
                Ok(readed) if readed > 0 => read_from_socked_resp(&response),
                Ok(_) => {
                    eprintln!("No response from daemon");
                    None
                }
                Err(error) => {
                    eprintln!("Read failed: {error}");
                    None
                }
            };
        } else {
            eprintln!("Socket write failed!");
        }
    }
    return None;
}

/// Deserializes incomming bytes in order to return
/// a `DaemonResponse`. None is returned if deserializing failed
fn read_from_socked_resp(bytes: &[u8]) -> Option<DaemonResponse> {
    match bincode::deserialize::<DaemonResponse>(bytes) {
        Ok(res) => {
            println!("RES: {:?}", res);
            return Some(res);
        }
        Err(e) => {
            println!("RES ERROR: {}", e);
            return None;
        }
    }
}

/// Deserializes incomming bytes in order to return
/// a `DaemonCommand`. None is returned if deserializing failed
#[allow(dead_code)]
pub fn read_from_socket_req(bytes: &[u8]) -> Option<DaemonCommand> {
    match bincode::deserialize::<DaemonCommand>(bytes) {
        Ok(res) => {
            println!("REQ: {:?}", res);
            return Some(res);
        }
        Err(e) => {
            println!("REQ ERROR: {}", e);
            return None;
        }
    }
}
