---
applyTo: "razer_control_gui/kde-widget/**"
description: "Use when editing the KDE Plasma 6 widget (C++/QML/CMake). Covers Qt6/KF6 conventions, the JSON IPC protocol, QML UI patterns, and build system. Distinct from the Rust codebase."
---
# KDE Widget Guidelines

This is a **Plasma 6 system tray applet** — completely separate from the Rust codebase. It uses C++17, Qt6, KDE Frameworks 6, and QML.

## Architecture

- `src/razercontrolwidget.{h,cpp}` — Main `Plasma::Applet` subclass. Lifecycle, config, auto-start, Q_PROPERTY exports to QML.
- `src/daemoncommunicator.{h,cpp}` — Socket client. JSON commands over `QLocalSocket` to `~/.local/share/razer-daemon.sock`.
- `package/contents/ui/main.qml` — Full QML UI (compact panel icon + expanded popup). Uses Kirigami and QtQuick.Controls.
- `package/contents/config/main.xml` — KDE config schema (kcfg format).
- `package/metadata.json` — Plasma 6 plugin metadata.

## IPC protocol — JSON, not bincode

The widget talks to the daemon via **JSON over a local Unix socket** (`~/.local/share/razer-daemon.sock`), NOT the bincode protocol used by the Rust CLI/GUI (`/tmp/razercontrol-socket`). Commands are newline-delimited JSON objects:
```json
{"command": "GetDeviceName"}
```

## Build

```bash
cd razer_control_gui/kde-widget
mkdir build && cd build
cmake .. -DCMAKE_INSTALL_PREFIX=$(kf6-config --prefix)
make && make install

# Or use the install script:
./install-plasmoid.sh
```

## QML conventions

- Import style: bare module names (`import QtQuick`, not `import QtQuick 2.15`) — Plasma 6 / Qt6 convention
- Use `Kirigami.Units` for spacing and sizing, never hardcode pixel values (except icon sizes like 16/18px)
- Use `Kirigami.Theme` for colors — respect system theme, no hardcoded colors
- Temperature color thresholds: >90°C = `negativeTextColor`, >75°C = `neutralTextColor`, else `positiveTextColor`
- Placeholder value `"--"` means "no data yet" — always check before displaying

## C++ conventions

- `K_PLUGIN_CLASS_WITH_JSON` macro registers the applet with Plasma
- Member variables prefixed with `m_` (e.g. `m_socket`, `m_deviceName`)
- Use `KConfigGroup` for reading/writing widget settings, not raw QSettings
- Config entries: `StartMinimized`, `EnableAutoStart`, `ShowBatteryPercentage`, `RefreshInterval` (1-10 sec, default 2)
- Signal/slot connections use Qt5-style pointer-to-member syntax

## Documentation

- [ARCHITECTURE.md](razer_control_gui/kde-widget/ARCHITECTURE.md) — System design, state machines, deployment
- [CONFIGURATION.md](razer_control_gui/kde-widget/CONFIGURATION.md) — Full settings reference, troubleshooting
- [QUICKSTART.md](razer_control_gui/kde-widget/QUICKSTART.md) — 5-minute setup guide
