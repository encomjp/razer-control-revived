# File Manifest - KDE Widget Installation

## Directory Structure

```
razer_control_gui/kde-widget/
├── Documentation
│   ├── README.md                    # Full technical documentation
│   ├── QUICKSTART.md               # Quick setup guide
│   ├── CONFIGURATION.md            # Configuration options
│   ├── ARCHITECTURE.md             # System design and architecture
│   ├── SUMMARY.md                  # This summary
│   ├── FILES.md                    # File manifest (this file)
│   └── install.sh                  # Installation script
│
├── Build Configuration
│   └── CMakeLists.txt              # CMake build configuration
│
├── Source Code (src/)
│   ├── razercontrolwidget.h        # Main widget header file
│   ├── razercontrolwidget.cpp      # Main widget implementation
│   ├── daemoncommunicator.h        # Daemon communication header
│   ├── daemoncommunicator.cpp      # Daemon communication implementation
│   └── resources.qrc               # Qt resource file
│
└── Package Contents (package/)
    ├── metadata.json               # KDE Plasmoid metadata
    └── contents/
        ├── ui/
        │   └── main.qml            # Widget UI (QML)
        └── config/
            └── main.xml            # Configuration schema
```

## File Descriptions

### 📖 Documentation Files

#### README.md (660 lines)
- **Purpose**: Complete technical reference
- **Contents**:
  - Feature overview
  - Installation instructions (with prerequisites)
  - Build and compilation
  - Auto-start setup
  - Command-line usage
  - Daemon communication protocol
  - Development guidelines
  - Troubleshooting guide
- **Audience**: Developers, advanced users
- **Read Time**: 15-20 minutes

#### QUICKSTART.md (100 lines)
- **Purpose**: Get up and running in 5 minutes
- **Contents**:
  - One-line installation command
  - Widget interaction basics
  - Configuration essentials
  - Common troubleshooting
- **Audience**: New users
- **Read Time**: 3-5 minutes

#### CONFIGURATION.md (350 lines)
- **Purpose**: Complete settings reference
- **Contents**:
  - Initial setup guide
  - Using the widget (left/right click)
  - Configuration options breakdown
  - Auto-start detailed setup
  - Panel positioning
  - Daemon integration details
  - Troubleshooting by symptom
  - Advanced customization
  - Widget integration with main app
- **Audience**: Regular users, power users
- **Read Time**: 10-15 minutes

#### ARCHITECTURE.md (400 lines)
- **Purpose**: System design documentation
- **Contents**:
  - System layout diagram
  - Widget interaction diagrams
  - Application flow charts
  - Component architecture
  - State diagrams
  - Configuration workflow
  - Component file reference
  - Usage scenarios
  - Deployment checklist
- **Audience**: Developers, system administrators
- **Read Time**: 15-20 minutes

#### SUMMARY.md (300 lines)
- **Purpose**: High-level overview
- **Contents**:
  - What was created
  - Package contents
  - Key features
  - Installation steps
  - Usage guide
  - Configuration options
  - How it works
  - Common tasks
  - Technical details
  - Requirements
  - Troubleshooting quick reference
  - Learning path
  - Checklists
- **Audience**: All users
- **Read Time**: 10 minutes

#### FILES.md (this file)
- **Purpose**: File manifest and reference
- **Contents**:
  - Directory structure
  - File descriptions
  - Line counts and purposes
  - File relationships
  - Usage patterns
- **Audience**: Developers, documentation
- **Read Time**: 5-10 minutes

### 🔧 Installation & Build

#### install.sh (200 lines)
- **Purpose**: Automated installation script
- **Features**:
  - Dependency checking (CMake, Qt, KDE Frameworks)
  - Build directory setup
  - CMake configuration
  - Compilation with parallel make
  - Installation with optional sudo
  - System cache rebuild
  - Auto-start entry creation
  - Post-install guidance
  - Optional Plasma restart
- **Usage**: `bash install.sh`
- **Requirements**: Linux, standard build tools
- **Execution Time**: 5-15 minutes (depends on system)

#### CMakeLists.txt (35 lines)
- **Purpose**: Build configuration
- **Contents**:
  - C++ standard (C++17)
  - MOC/RCC settings
  - Dependencies (Qt6, KDE Frameworks)
  - Library compilation
  - Installation targets
- **Format**: CMake 3.16+
- **Used By**: install.sh, developers

### 💻 Source Code

#### razercontrolwidget.h (50 lines)
- **Purpose**: Main widget class declaration
- **Declares**:
  - `RazerControlWidget` class (extends Plasma::Applet)
  - Properties: deviceName, batteryPercentage, isMinimized
  - Slots: updateDeviceInfo(), openSettings(), minimizeApp()
  - Configuration management functions
  - Signal definitions
- **Used By**: razercontrolwidget.cpp, CMake build

#### razercontrolwidget.cpp (200 lines)
- **Purpose**: Main widget implementation
- **Implements**:
  - Widget initialization and lifecycle
  - Daemon communication
  - Auto-start configuration setup
  - Configuration UI creation
  - Menu and action handling
  - Battery/device info updates
  - Process management (launch/minimize)
- **Depends On**: daemoncommunicator.h, Qt, KDE Frameworks
- **Qt Magic**: K_PLUGIN_CLASS_WITH_JSON macro

#### daemoncommunicator.h (25 lines)
- **Purpose**: Daemon communication interface
- **Declares**:
  - `DaemonCommunicator` class
  - Socket connection management
  - Command sending interface
  - Response parsing functions
  - Device info retrieval (name, battery %)
- **Used By**: razercontrolwidget.cpp

#### daemoncommunicator.cpp (120 lines)
- **Purpose**: Daemon communication implementation
- **Implements**:
  - Local socket connection to daemon
  - JSON command sending
  - Response parsing
  - Device name retrieval
  - Battery percentage retrieval
  - Generic command sending
  - Connection handling and error recovery
- **Protocol**: JSON over local socket
- **Socket Location**: ~/.local/share/razer-daemon.sock

#### resources.qrc (5 lines)
- **Purpose**: Qt resource file for QML
- **Contents**: QML file path references
- **Format**: Qt resource XML format
- **Used By**: CMake resource compilation

### 📦 Package Contents

#### metadata.json (25 lines)
- **Purpose**: KDE Plasmoid plugin metadata
- **Contains**:
  - Plugin name: "Razer Control"
  - Icon: power management icon
  - Description: system tray control
  - Service type: Plasma/Applet
  - Author: Encomjp
  - Version: 0.2.0
  - License: GPLv2+
  - Website: GitHub repository link
- **Format**: JSON
- **Used By**: KDE Plasma plugin system

#### main.qml (150 lines)
- **Purpose**: Widget visual interface
- **Implements**:
  - Compact representation (system tray icon)
  - Full representation (expanded widget)
  - Context menu with all actions
  - Tooltip display
  - Mouse interaction handlers
  - Device/battery info display
- **Language**: QML (Qt Modeling Language)
- **Features**:
  - Icon display with tooltip
  - Right-click context menu
  - Left-click to open settings
  - Status information display
  - Hover detection

#### main.xml (30 lines)
- **Purpose**: Configuration schema
- **Defines**:
  - StartMinimized (bool)
  - EnableAutoStart (bool)
  - ShowBatteryPercentage (bool)
  - RefreshInterval (int, 1-10 seconds)
- **Format**: KDE configuration XML
- **Used By**: KDE config system
- **Storage**: ~/.config/plasmarc

## File Statistics

| Category | Count | Lines | Purpose |
|----------|-------|-------|---------|
| Documentation | 6 | 2000+ | Setup, usage, reference |
| Build/Config | 2 | 40 | CMake, QRC |
| C++ Source | 4 | 400 | Widget logic |
| QML/Config | 2 | 180 | UI and schema |
| Scripts | 1 | 200 | Installation automation |
| **Total** | **15** | **2800+** | Complete KDE widget |

## File Dependencies

```
cmake compilation
├─ CMakeLists.txt
│  ├─ src/razercontrolwidget.h
│  ├─ src/razercontrolwidget.cpp
│  │  ├─ src/daemoncommunicator.h
│  │  └─ src/daemoncommunicator.cpp
│  ├─ src/resources.qrc
│  │  └─ package/contents/ui/main.qml
│  └─ package/metadata.json
│
└─ package/contents/config/main.xml
```

Installation process:
```
install.sh
├─ Check dependencies
├─ Create build directory
├─ Run cmake (reads CMakeLists.txt)
├─ Run make (compiles C++ and QML)
├─ Run make install (installs compiled files)
├─ Create ~/.config/autostart/razer-settings.desktop
└─ Rebuild KDE cache
```

## Usage Workflow

### First-Time Setup
1. Read: QUICKSTART.md (5 min)
2. Run: `bash install.sh` (5-10 min)
3. Restart Plasma when prompted
4. Add widget to panel

### Regular Usage
- Interact with widget in system tray
- Right-click for quick menu
- Widget automatically updates battery status

### Configuration
1. Right-click widget → "Configuration"
2. Modify settings from main.xml schema
3. Changes saved to ~/.config/plasmarc
4. Applied immediately

### Troubleshooting
1. Check: CONFIGURATION.md (Troubleshooting section)
2. Run diagnostic commands from CONFIGURATION.md
3. View logs: `journalctl -u razercontrol`
4. Check file: `~/.config/autostart/razer-settings.desktop`

## Integration Points

### With Razer Control Daemon
- **Communication**: JSON over local socket
- **Location**: `~/.local/share/razer-daemon.sock`
- **Files**: daemoncommunicator.h/cpp

### With KDE Plasma
- **Integration**: CMake, KDE Frameworks
- **Plugin**: metadata.json
- **Config**: main.xml (KDE config system)

### With System
- **Auto-start**: ~/.config/autostart/
- **Settings**: ~/.config/plasmarc
- **Installation**: /usr/local/share/plasma/plasmoids/

## Development Notes

### Building Locally
```bash
cd kde-widget/build
cmake ..
make
# Do NOT run sudo make install to test locally
# Use: export LD_LIBRARY_PATH=src/:$LD_LIBRARY_PATH
```

### Modifying QML
- Edit: package/contents/ui/main.qml
- Rebuild: make (in build directory)
- Test: kquitapp6 plasmashell; sleep 2; kstart6 plasmashell &

### Modifying C++
- Edit: src/razercontrolwidget.cpp or daemoncommunicator.cpp
- Rebuild: make
- Install: sudo make install
- Cache: kbuildsycoca6

### Adding New Features
1. Update main.xml for new settings
2. Add C++ properties to razercontrolwidget.h
3. Implement in razercontrolwidget.cpp
4. Update QML in main.qml
5. Rebuild and test

## Version Information

- **Widget Version**: 0.2.0 (from metadata.json)
- **Razer Control Version**: 0.2.0
- **License**: GPLv2+
- **Author**: Encomjp
- **Created**: February 2026

## Total Package Size

- **Source**: ~500 KB (uncompiled)
- **Compiled**: ~2-5 MB (depends on system)
- **Installation**: ~10 MB (includes Qt/KDE copies)

## Summary

This KDE widget package contains everything needed to add system tray control for Razer laptops to KDE Plasma. The package includes:
- Complete source code (C++, QML)
- Build configuration (CMake)
- Automated installation script
- Comprehensive documentation
- Auto-start configuration support
- Settings management

All files work together to create a seamless, professional KDE Plasma applet that integrates with the Razer Control daemon and provides quick access to laptop controls from the system tray.
