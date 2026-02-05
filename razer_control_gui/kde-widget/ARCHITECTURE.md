# KDE Widget Architecture & Usage Guide

## System Layout

```
Desktop Screen
┌──────────────────────────────────────────────────────────────┐
│                                                               │
│                   Your Applications                           │
│                      (Workspace)                              │
│                                                               │
├──────────────────────────────────────────────────────────────┤
│ ≡ Activities  [Open Windows]  [System Tray]    🕐 [Razer🎮] ✕ │
│                                            ↑
│                              Razer Control Widget
│                              (Bottom-Right)
└──────────────────────────────────────────────────────────────┘

System Tray Icons (Right side of panel):
┌─────────────────────────────────┐
│ [Network] [Audio] [Razer 🎮] ✕  │
│                     ↑
│              Our Widget
└─────────────────────────────────┘
```

## Widget Interactions

### Left-Click
```
[Click Widget] → Opens Razer Settings Window
                    ↓
            ┌───────────────────┐
            │  Razer Settings   │
            │  ─────────────────│
            │  Fan Control      │
            │  Power Profiles   │
            │  RGB Effects      │
            │  Battery Health   │
            └───────────────────┘
```

### Right-Click (Context Menu)
```
[Right-Click] → Shows Quick Menu
                    ↓
    ┌──────────────────────────────┐
    │  Open Settings               │
    │  ─────────────────────────── │
    │  Fan Control                 │
    │  Power Profiles              │
    │  RGB Control                 │
    │  Battery Health              │
    │  ─────────────────────────── │
    │  Minimize                    │
    │  Configuration               │
    │  Exit                        │
    └──────────────────────────────┘
```

### Hover (Tooltip)
```
Razer Control
Battery: 85%
Click to open settings
```

## Application Flow

```
System Boots
    ↓
[Auto-start enabled?]
    ├─Yes→ ~/.config/autostart/razer-settings.desktop executes
    │       ↓
    │   [Start minimized?]
    │       ├─Yes→ App starts in system tray (hidden)
    │       └─No → Full window opens
    │
    └─No → No automatic startup

At Runtime:
    ↓
Razer Control Widget (in system tray)
    ├─ Left-click → Open Full GUI
    ├─ Right-click → Show Quick Menu
    │   ├─ Fan Control (opens fan tab)
    │   ├─ Power Profiles (opens power tab)
    │   ├─ RGB Control (opens rgb tab)
    │   ├─ Battery Health (opens battery tab)
    │   └─ Minimize (hides window)
    └─ Hover → Show device info tooltip

Configuration:
    ├─ Right-click → "Configuration"
    │   ├─ Enable/disable auto-start
    │   ├─ Enable/disable start minimized
    │   ├─ Show/hide battery % in widget
    │   └─ Adjust refresh interval
    └─ Stored in: ~/.config/...
```

## Component Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Razer Control KDE Widget                    │
│  ─────────────────────────────────────────────────────  │
│                                                          │
│  ┌──────────────────────────────────────────────────┐   │
│  │     QML User Interface (QML Engine)              │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │ System Tray Icon (Compact)                  │ │   │
│  │  │ ┌─────────────────┐                         │ │   │
│  │  │ │    [Razer 🎮]   │ ← Shows power icon      │ │   │
│  │  │ │   Battery: 85%  │ ← Tooltip info         │ │   │
│  │  │ └─────────────────┘                         │ │   │
│  │  │        │                                    │ │   │
│  │  │        ├─Left-click → Open Settings        │ │   │
│  │  │        └─Right-click → Context Menu        │ │   │
│  │  │                                            │ │   │
│  │  │ Context Menu (Expandable)                  │ │   │
│  │  │ ┌─────────────────────────────────────┐   │ │   │
│  │  │ │ Open Settings                       │   │ │   │
│  │  │ │ Fan Control                         │   │ │   │
│  │  │ │ Power Profiles                      │   │ │   │
│  │  │ │ RGB Control                         │   │ │   │
│  │  │ │ Battery Health                      │   │ │   │
│  │  │ │ ─────────────────────────────────── │   │ │   │
│  │  │ │ Minimize                            │   │ │   │
│  │  │ │ Configuration                       │   │ │   │
│  │  │ │ Exit                                │   │ │   │
│  │  │ └─────────────────────────────────────┘   │ │   │
│  │  │                                            │ │   │
│  │  │ Expanded Widget View (if not in tray)     │ │   │
│  │  │ ┌──────────────────────────────────────┐  │ │   │
│  │  │ │ Razer Control                        │  │ │   │
│  │  │ │ Device: Blade 15 (2023)              │  │ │   │
│  │  │ │ Battery: 85%                         │  │ │   │
│  │  │ │                                      │  │ │   │
│  │  │ │ [Open Settings] [Minimize]           │  │ │   │
│  │  │ └──────────────────────────────────────┘  │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  │                                                   │   │
│  ├──────────────────────────────────────────────────┤   │
│  │      C++ Backend (Widget Logic)                  │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │ RazerControlWidget (Main Applet Class)      │ │   │
│  │  │  • Manages widget lifecycle                 │ │   │
│  │  │  • Handles configuration                    │ │   │
│  │  │  • Updates from daemon                      │ │   │
│  │  │  • Auto-start setup                         │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │ DaemonCommunicator (Socket Client)          │ │   │
│  │  │  • Connect to daemon socket                 │ │   │
│  │  │  • Send JSON commands                       │ │   │
│  │  │  • Receive device info                      │ │   │
│  │  │  • Parse responses                          │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  └──────────────────────────────────────────────────┘   │
│                          │                               │
│                          ↓                               │
│  ┌──────────────────────────────────────────────────┐   │
│  │   Configuration Storage & Auto-Start             │   │
│  │  • ~/.config/plasmarc (widget config)            │   │
│  │  • ~/.config/autostart/razer-settings.desktop    │   │
│  │  • ~/.config/kf6rc (KDE config)                  │   │
│  └──────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
         │
         │ Local Socket IPC (JSON)
         ↓
┌─────────────────────────────────┐
│    Razer Control Daemon          │
│  • Device communication          │
│  • State management              │
│  • Power profile control         │
│  • Fan control                   │
│  • RGB effects                   │
│  • Battery health                │
└─────────────────────────────────┘
         │
         │ USB/HID
         ↓
┌─────────────────────────────────┐
│    Razer Laptop Hardware         │
│  • USB Device (1532:xxxx)        │
│  • Fan controllers               │
│  • RGB LED                       │
│  • Power management              │
│  • Battery interface             │
└─────────────────────────────────┘
```

## Widget State Diagram

```
        ┌─────────────────┐
        │   Application   │
        │   Not Running   │
        └────────┬────────┘
                 │ System boots & auto-start enabled
                 ↓
        ┌─────────────────────┐
        │  Daemon Starts      │
        │  (systemd service)  │
        └────────┬────────────┘
                 │
                 ↓
        ┌─────────────────────┐
        │  Widget Starts      │
        │  (in system tray)   │
        └────────┬────────────┘
                 │
        ┌────────┴────────┐
        ↓                 ↓
    Minimized         Visible
    (Hidden in        (Window
     system tray)      open)
        │               │
        ├─Click─────────┤
        ↓               │
        ├─────────────→ Open Window
        ↓               │
    Minimized ←────────┘
    (again)            Close

    Right-click menu available in any state
    ├─ Open Settings
    ├─ Minimize
    ├─ Configuration
    └─ Exit
```

## Configuration Workflow

```
User opens Configuration
        ↓
    ┌─────────────────────────────┐
    │   Configuration Dialog       │
    │  ┌───────────────────────┐  │
    │  │ □ Start minimized     │  │
    │  │ ☑ Auto-start on boot │  │
    │  │ ☑ Show battery %      │  │
    │  │                       │  │
    │  │ Refresh: [2] seconds  │  │
    │  │                       │  │
    │  │  [OK]     [Cancel]    │  │
    │  └───────────────────────┘  │
    └─────────────────────────────┘
              │
              ↓ User clicks OK
    ┌──────────────────────────────┐
    │ Save Configuration            │
    │ ~/.config/plasmarc            │
    │ ~/.config/autostart/...       │
    └──────────────────────────────┘
              │
              ↓ Apply changes
    ┌──────────────────────────────┐
    │ Widget Updated               │
    │ (Timer interval, start mode) │
    └──────────────────────────────┘
```

## Quick Reference: Widget Files

| File | Purpose |
|------|---------|
| `src/razercontrolwidget.cpp` | Main widget logic, daemon communication |
| `src/daemoncommunicator.cpp` | Socket communication with daemon |
| `package/contents/ui/main.qml` | Visual interface (QML) |
| `package/metadata.json` | Widget metadata (name, version, icon) |
| `package/contents/config/main.xml` | Configuration schema |
| `CMakeLists.txt` | Build configuration |
| `install.sh` | Installation script |

## Usage Scenarios

### Scenario 1: User Just Booted System
```
User boots system
    ↓
Auto-start entry executes: razer-settings --minimized
    ↓
Daemon starts in background
    ↓
Widget appears in system tray (minimized)
    ↓
User can click widget for full GUI or right-click for quick menu
```

### Scenario 2: User Changes Fan Settings via Widget
```
Right-click widget → "Fan Control"
    ↓
Razer Settings opens (or focuses existing window)
    ↓
User adjusts fan settings
    ↓
Settings sent to daemon
    ↓
Daemon applies to hardware
    ↓
User closes window → app minimizes to tray (if enabled)
```

### Scenario 3: User Wants Quick Battery Health Check
```
Hover over widget icon
    ↓
Tooltip shows: "Device: Blade 15, Battery: 85%"
    ↓
User sees battery is healthy, no action needed
```

### Scenario 4: User Configures Widget
```
Right-click widget → "Configuration"
    ↓
Opens configuration dialog
    ↓
User toggles settings:
  ✓ Disable "Start minimized"
  ✓ Set refresh to 5 seconds
    ↓
Click OK
    ↓
Settings saved to ~/.config/
    ↓
Widget applies new settings immediately
```

## Deployment Checklist

- [x] Source files created (C++ and QML)
- [x] CMake build configuration
- [x] Metadata and configuration XML
- [x] Installation script with dependency checking
- [x] Auto-start configuration
- [x] Documentation (README, CONFIGURATION, QUICKSTART)
- [x] Widget properly packaged for KDE Plasma
- [x] Error handling for daemon communication
- [x] Configuration persistence
- [x] Tooltip and menu integration

## Next: Installation

Run the installer to build and install the widget:

```bash
cd razer_control_gui/kde-widget
bash install.sh
```

See [QUICKSTART.md](QUICKSTART.md) for immediate usage instructions.
