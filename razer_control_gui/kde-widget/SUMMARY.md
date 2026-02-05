# Razer Control KDE Widget - Complete Package Summary

## ❤️ Support This Project

If you find this project useful, please consider donating to support continued development and add support for more Razer blade models:

[![Donate](https://img.shields.io/badge/Donate-PayPal-blue.svg)](https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A)

## ⚠️ Important Disclaimer

**Tested on:** Fedora Linux (as of February 2026)

This KDE widget has been primarily tested on Fedora. It should work on Ubuntu and similar Linux distributions, but **no guarantees are given**. If you experience issues:
1. Report them in the [Issues](https://github.com/encomjp/razer-control/issues) section
2. Provide your distribution name and error details
3. I will work with you to add support

## What Was Created

A complete KDE Plasma widget (panel applet) for controlling your Razer laptop from the system tray.

## 📦 Package Contents

### Core Files Created

```
kde-widget/
├── README.md                          # Full technical documentation
├── QUICKSTART.md                      # Fast setup guide (START HERE)
├── CONFIGURATION.md                   # Settings and options
├── ARCHITECTURE.md                    # System design and layouts
│
├── install.sh                         # Automated installation script
├── CMakeLists.txt                     # Build configuration
│
├── src/
│   ├── razercontrolwidget.h/.cpp      # Main widget C++ implementation
│   ├── daemoncommunicator.h/.cpp      # Daemon communication layer
│   └── resources.qrc                  # Qt resource file
│
└── package/
    ├── metadata.json                  # Widget metadata
    └── contents/
        ├── ui/main.qml                # Widget visual interface
        └── config/main.xml            # Configuration schema
```

## 🎯 Key Features

### Widget Appearance & Location
- 📍 **System Tray** - Appears in bottom-right corner of screen
- 🔌 **Persistent** - Always accessible without opening main window
- 💻 **Memory Efficient** - Lightweight indicator

### Interaction Methods

**Left-Click**
- Opens full Razer Settings application
- Access to all controls and settings

**Right-Click (Quick Menu)**
- Fan Control - Quick access to fan curves
- Power Profiles - Switch between Gaming/Balanced/Silent
- RGB Control - Keyboard lighting settings
- Battery Health - Battery charge limiter (BHO)
- Minimize - Hide application window
- Configuration - Widget settings
- Exit - Close application

**Hover (Tooltip)**
- Shows device name
- Displays current battery percentage
- Quick status check

### Configuration Features
- ✓ **Auto-Start on Boot** - Automatically launch daemon + GUI
- ✓ **Start Minimized** - Application starts in tray, not full window
- ✓ **Live Battery Monitoring** - Refreshes battery % every 2 seconds (configurable)
- ✓ **Battery % Display** - Shows battery in widget tooltip
- ✓ **Refresh Interval** - Adjustable update frequency (1-10 seconds)

### Auto-Start Functionality
- Creates desktop entry: `~/.config/autostart/razer-settings.desktop`
- Automatically enabled during installation
- Can be disabled in widget settings or System Settings
- Supports "start minimized" flag for clean boot

## 🚀 Installation Steps

### Quick Install (Automatic)
```bash
cd razer_control_gui/kde-widget
bash install.sh
```

The script will:
1. Check for required dependencies
2. Build from source using CMake
3. Install to KDE installation directory
4. Set up auto-start configuration
5. Optionally restart Plasma

### Manual Installation (Advanced)
```bash
cd razer_control_gui/kde-widget
mkdir build && cd build
cmake ..
make
sudo make install
kbuildsycoca6  # or kbuildsycoca5 for KDE 5
```

## 📖 Usage

### Adding Widget to Panel

1. **Right-click your panel** (bottom of screen)
2. Select **"Edit Panel"** or **"Add Widgets"**
3. Click **"+"** button
4. Search for **"Razer Control"**
5. Click to add to panel

### Using the Widget

The widget icon appears in your system tray:

```
Panel: [Other Items] [Clock] [Razer 🎮] ✕
                             ↑
                        Our Widget
```

**Actions:**
- **Left-click** → Open settings window
- **Right-click** → Show quick control menu
- **Hover** → See battery & device info

## 🔧 Configuration Options

Access by: Right-click widget → "Configuration"

| Setting | Options | Purpose |
|---------|---------|---------|
| Start Minimized | On/Off | Launch app hidden in tray |
| Auto-Start Boot | On/Off | Launch daemon + GUI on boot |
| Show Battery % | On/Off | Display battery in tooltip |
| Refresh Interval | 1-10 sec | Update frequency for battery/device info |

All settings are saved to `~/.config/plasmarc`

## 🔄 How It Works

```
System Boot
    ↓
Auto-start enabled → ~/.config/autostart/razer-settings.desktop
    ↓
Razer Control daemon starts
    ↓
Widget appears in system tray
    ↓
User can interact via:
  • Left-click → Open full GUI
  • Right-click → Show quick menu
  • Hover → See status tooltip
    ↓
Configuration persisted to disk
    ↓
On next boot, settings are remembered
```

## 💡 Common Tasks

### Enable Auto-Start
1. Right-click widget → "Configuration"
2. Check "Auto-start on system boot"
3. Click OK

### Start Application Minimized
1. Right-click widget → "Configuration"
2. Check "Start application minimized on boot"
3. Click OK
4. Next boot, app starts hidden in tray

### Change Refresh Speed
1. Right-click widget → "Configuration"
2. Adjust "Refresh interval" (1-10 seconds)
3. Lower = more responsive but uses more CPU
4. Default is 2 seconds (recommended)

### Access Fan Control Quickly
1. Right-click widget icon
2. Select "Fan Control" from menu
3. Full settings window opens on that tab

### View Battery Status
1. Hover over widget icon
2. Tooltip shows: "Razer Control - Battery: 85%"
3. No need to open full window for quick check

## 🛠️ Technical Details

### Communication Protocol
- **Type**: Local socket IPC (Inter-Process Communication)
- **Location**: `~/.local/share/razer-daemon.sock`
- **Format**: JSON-based commands and responses
- **Examples**:
  ```json
  {"command": "GetDeviceName"}
  {"command": "GetBattery"}
  {"command": "SetFan", "rpm": 3500}
  ```

### Configuration Files
- **Widget Settings**: `~/.config/plasmarc`
- **Auto-Start**: `~/.config/autostart/razer-settings.desktop`
- **KDE Config**: `~/.config/kf6rc` (or kf5rc)

### System Integration
- **Widget Manager**: KDE Plasma
- **Build System**: CMake
- **Language**: C++17, QML, Qt6
- **Dependencies**: KDE Frameworks, Qt6 Core/GUI/DBus

## 📋 Requirements

### Build Requirements
- CMake 3.16+
- GCC/Clang C++17 compiler
- Qt6 or Qt5.15+
- KDE Frameworks (KF6 or KF5)
- pkg-config

### Runtime Requirements
- KDE Plasma 5.27+ or KDE Plasma 6.x
- Razer Control daemon installed and running
- Linux system

### Optional
- systemd (for enhanced auto-start)

## 🆘 Troubleshooting

### Widget not appearing in add-widgets list?
```bash
# Rebuild KDE cache
kbuildsycoca6  # (or kbuildsycoca5 for KDE 5)

# Restart Plasma
kquitapp6 plasmashell; sleep 2; kstart6 plasmashell &
```

### Shows "Detecting..." for device?
```bash
# Check daemon status
systemctl status razercontrol
# or
pgrep daemon

# View logs
journalctl -u razercontrol -n 50
```

### Auto-start not working?
```bash
# Check file exists
cat ~/.config/autostart/razer-settings.desktop

# Fix permissions
chmod 644 ~/.config/autostart/razer-settings.desktop

# Verify in System Settings
# System Settings → Startup and Shutdown → Desktop Session
```

### Context menu doesn't respond?
```bash
# Close existing Razer Settings windows
pkill razer-settings

# Restart the app
razer-settings &
```

## 📚 Documentation Files

| File | Content |
|------|---------|
| **README.md** | Full technical reference and API details |
| **QUICKSTART.md** | Fast setup (5 minutes to working widget) |
| **CONFIGURATION.md** | All settings, options, and customization |
| **ARCHITECTURE.md** | System design, diagrams, data flow |
| **This file** | Overview and quick reference |

## 🎓 Learning Path

1. **Just want to use it?** → Read [QUICKSTART.md](QUICKSTART.md)
2. **Want full details?** → Read [README.md](README.md)
3. **Need to configure?** → Read [CONFIGURATION.md](CONFIGURATION.md)
4. **Understanding the design?** → Read [ARCHITECTURE.md](ARCHITECTURE.md)
5. **Modifying code?** → Check [README.md](README.md) development section

## 📋 Installation Checklist

- [ ] Run `bash install.sh`
- [ ] Confirm all dependencies installed
- [ ] Choose to restart Plasma (recommended)
- [ ] Right-click panel → Add Widgets
- [ ] Search and add "Razer Control"
- [ ] Configure settings if needed
- [ ] Test left-click (opens settings)
- [ ] Test right-click (shows menu)
- [ ] Hover to see tooltip
- [ ] Reboot to verify auto-start works

## 🚀 Next Steps

1. **Install**: Run `bash install.sh` in the kde-widget directory
2. **Add to Panel**: Right-click your Plasma panel and add the widget
3. **Configure**: Right-click widget and customize settings
4. **Enjoy**: Use the quick menu for fast Razer control

## 📞 Support

For issues:
1. Check widget logs: `journalctl -n 100`
2. Verify daemon: `systemctl status razercontrol`
3. Check configuration: `cat ~/.config/autostart/razer-settings.desktop`
4. Report to: [Razer Control GitHub](https://github.com/encomjp/razer-control)

## 📄 License

GPLv2+ - Same as Razer Control main project

---

**Installation Time**: ~5-10 minutes (depends on build speed)  
**Configuration Time**: ~2 minutes  
**Total Setup**: ~15 minutes from start to fully working widget
