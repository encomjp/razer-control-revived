# Changelog

## 0.3.0-rc1 (2026-03-27)

### Bug Fixes

- **KDE Plasma 6: window not receiving focus on Wayland** — On Plasma 6 (Wayland),
  the Razer Settings window would not receive focus or appear in Alt+Tab. Instead it
  showed as "demanding attention" (orange highlight) in the Task Manager widget.
  - **Root cause**: KWin matches windows to desktop entries using the Wayland `app_id`.
    GTK4 sets this to the GApplication ID (`com.encomjp.razer-settings`), but the
    installed desktop file was named `razer-settings.desktop`. KWin could not find
    `com.encomjp.razer-settings.desktop`, so startup notification failed.
  - **Fix**: Renamed desktop file to `com.encomjp.razer-settings.desktop` to match the
    GApplication ID. Added `StartupNotify=true` and `StartupWMClass=com.encomjp.razer-settings`
    for proper window-to-launcher association.
  - Updated all install scripts, CI workflows, packaging (RPM, DEB, AppImage, Nix, tarball).

- **Documentation: git clone URL** — Removed unnecessary `.git` suffix from clone
  commands in README and razer_control_gui README.

## 0.2.9

- Battery health optimization and charge limit controls
- System tray with live sensor monitoring (CPU/GPU temp, power, utilization)
- Close-to-tray behavior (app stays alive in background)
- KDE Plasma widget live sync improvements

## 0.2.6

- Fix panic inside panic hook during socket cleanup
- Fix potential panic from system time anomaly in get_millis()
- Fix panic during graceful shutdown socket cleanup

## 0.2.5

- Security: restrict daemon socket permissions to owner-only (0600)
- Fix buffer overflow in set_standard_effect params
- Fix array index panics in keyboard effect constructors
- Add bounds validation for AC state index in daemon commands
- Fix mutex poison cascade crashes in daemon threads
- Fix D-Bus connection panics with graceful fallback
- Fix HOME environment variable panic in config
- Fix brightness overflow when value exceeds 100
- Replace all unsafe JSON unwrap chains with proper error handling
- Fix deprecated glib::clone! syntax warnings
- Clean up all 46 compiler warnings (zero warnings now)

## 0.2.4

- Add 12 new Razer laptop models (Blade 15/16/18 2023-2025, Stealth 2015/2016, Studio Edition)
- Settings persistence: all settings saved to config and restored on boot
- Live sync between KDE widget and GUI app (2-second polling)
- Fix KDE widget AC/battery profile mismatch
- Fix systemd user service (correct targets, binary paths, auto-create config dir)
- Fix DEB package systemd user service enablement

## 0.2.1

- UI rework: native libadwaita widgets (SwitchRow, ComboRow, AlertDialog)
- Simplified CSS with Razer green accent on libadwaita defaults
- Remove legacy unused source files
- Add .deb package and Nix flake CI support

## 0.2.0

- Migrate to GTK4 with libadwaita modern UI
- Add status bar monitoring
- Add AMD hardware support

## 0.1.0

- Initial release
