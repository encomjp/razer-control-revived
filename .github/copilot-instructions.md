# Project Guidelines

## Overview

Razer Control Revived — Linux userspace application for controlling Razer Blade laptops without kernel modules/DKMS. Three binaries (daemon, razer-settings GUI, razer-cli) communicating over a Unix socket with binary-serialized IPC (bincode).

## Architecture

```
┌──────────────┐  ┌──────────────┐  ┌────────────────┐
│  razer-cli   │  │razer-settings│  │  KDE widget    │
│  (CLI, clap) │  │ (GTK4+libadw)│  │(Plasma 6, C++) │
└──────┬───────┘  └──────┬───────┘  └──────┬─────────┘
       │ bincode         │ bincode         │ JSON
       └────────┬────────┘                 │
                ▼                          ▼
        ┌──────────────────────────────────────┐
        │           daemon (root)              │
        │  Unix socket: /tmp/razercontrol-socket│
        │  HID API → Razer hardware            │
        └──────────────────────────────────────┘
```

**Key boundaries:**
- `src/daemon/` — Privileged daemon: HID hardware control, config persistence, D-Bus integration (UPower, Mutter, login1)
- `src/razer-settings/` — GTK4 + libadwaita GUI (no direct hardware access)
- `src/cli/` — CLI via clap (no direct hardware access)
- `src/comms.rs` + `src/lib.rs` — Shared IPC types (`DaemonCommand`/`DaemonResponse` enums, bincode serialization)
- `kde-widget/` — Separate C++/Qt6/KDE Frameworks 6 Plasma applet (JSON over local socket `~/.local/share/razer-daemon.sock`)

**Config storage:** `~/.local/share/razercontrol/daemon.json` (dual AC/Battery power profiles)

**Device database:** [razer_control_gui/data/devices/laptops.json](razer_control_gui/data/devices/laptops.json) — array of `{name, vid, pid, features[], fan[min,max]}`

## Build and Test

```bash
# Build all binaries (daemon, razer-settings, razer-cli)
cd razer_control_gui && cargo build --release

# Install locally (binaries + udev + systemd)
./install-local.sh

# Install KDE widget
cd razer_control_gui/kde-widget && ./install-plasmoid.sh
```

**Rust edition:** 2024 (requires nightly)

**No automated test suite** — testing is manual per [razer_control_gui/TESTING_CHECKLIST.md](razer_control_gui/TESTING_CHECKLIST.md). `cargo build --release` in CI is the primary validation.

**Packaging:** RPM ([packaging/fedora/razercontrol.spec](packaging/fedora/razercontrol.spec)), DEB, tar.gz — all built in GitHub Actions on tag push (`v*`).

**Nix:** `flake.nix` provides a NixOS module with systemd + udev integration.

## Conventions

- **IPC protocol**: `DaemonCommand`/`DaemonResponse` enums in `comms.rs`, serialized with bincode over Unix stream. KDE widget uses a separate JSON protocol.
- **Error handling**: `Result<T, E>` throughout. GUI has `crash_with_msg()` and `setup_panic_hook()` in `error_handling.rs`. Avoid `unwrap()` in daemon code paths.
- **Power profiles**: Index 0 = AC, Index 1 = Battery in `Configuration.power[]` array. Power modes: 0=Balanced, 1=Gaming, 2=Creator.
- **Device features**: Tagged with `["fan", "logo", "boost", "per_key_rgb", "bho", "creator_mode"]` in `laptops.json`. Always check feature availability before sending hardware commands.
- **Dependencies**: GTK4 (`gtk4` crate), libadwaita, `hidapi` for HID, `dbus` for system bus, `ksni` for system tray. See [razer_control_gui/Cargo.toml](razer_control_gui/Cargo.toml).
- **Udev rules**: [data/udev/99-hidraw-permissions.rules](razer_control_gui/data/udev/99-hidraw-permissions.rules) grants userspace access to Razer HID devices. Must be updated when adding new device PIDs.

## Documentation

- [README.md](README.md) — Project overview, installation, supported devices
- [razer_control_gui/README.md](razer_control_gui/README.md) — Detailed build and development guide
- [razer_control_gui/kde-widget/ARCHITECTURE.md](razer_control_gui/kde-widget/ARCHITECTURE.md) — KDE widget system design
- [razer_control_gui/kde-widget/CONFIGURATION.md](razer_control_gui/kde-widget/CONFIGURATION.md) — Widget configuration reference
- [razer_control_gui/kde-widget/QUICKSTART.md](razer_control_gui/kde-widget/QUICKSTART.md) — Widget setup guide
- [razer_control_gui/TESTING_CHECKLIST.md](razer_control_gui/TESTING_CHECKLIST.md) — Manual testing procedures

## Adding a New Device

1. Add entry to `data/devices/laptops.json` with correct `vid`, `pid`, `features`, and `fan` range
2. Add matching PID to `data/udev/99-hidraw-permissions.rules`
3. Update `packaging/fedora/razercontrol.spec` version if needed
4. Test with `cargo build --release` — no unit tests exist, verify on hardware
