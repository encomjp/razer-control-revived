# 🎮 Razer Control KDE Widget - Start Here

Welcome! You now have a complete, production-ready KDE Plasma widget for controlling your Razer laptop from the system tray.

## ❤️ Support This Project

If you find this project useful, please consider donating to support continued development and add support for more Razer blade models:

[![Donate](https://img.shields.io/badge/Donate-PayPal-blue.svg)](https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A)

## ⚠️ Important Disclaimer

**Tested on:** Fedora Linux (as of February 2026)

This KDE widget has been primarily tested on Fedora. It should work on Ubuntu and similar Linux distributions, but **no guarantees are given**. If you experience issues on other distributions:
1. Check the [Issues](https://github.com/encomjp/razer-control/issues) page
2. Open a new issue with your distribution and error details
3. I will work with you to add support

## 🚀 Quick Start (5 minutes)

```bash
# 1. Navigate to widget directory
cd razer_control_gui/kde-widget

# 2. Run installation script
bash install.sh

# 3. Restart Plasma when prompted
# (or manually: kquitapp6 plasmashell && sleep 2 && kstart6 plasmashell &)

# 4. Right-click your panel → Add Widgets → Search "Razer Control" → Add

Done! Your widget is now in the system tray.
```

## 📋 What You Have

✅ Complete KDE Plasma widget (system tray applet)  
✅ C++ backend with daemon communication  
✅ QML user interface  
✅ Auto-start on boot capability  
✅ Minimized startup option  
✅ Context menu with quick controls  
✅ Battery monitoring and display  
✅ Configuration panel  
✅ Automated installation script  
✅ Comprehensive documentation  

## 🎯 Widget Features

### System Tray Display
```
[System Tray] ... [Razer 🎮] ✕
                      ↑
               Your new widget
```

### Left-Click
Opens full Razer Settings window

### Right-Click (Quick Menu)
- Open Settings
- Fan Control
- Power Profiles
- RGB Control
- Battery Health
- Minimize
- Configuration
- Exit

### Hover (Tooltip)
- Device name
- Current battery %
- Usage hint

## 📖 Documentation

### 🏃 For Impatient Users
**File**: [QUICKSTART.md](QUICKSTART.md)  
**Read Time**: 3-5 minutes  
**Content**: Installation and basic usage

### 👨‍💼 For Regular Users
**File**: [CONFIGURATION.md](CONFIGURATION.md)  
**Read Time**: 10-15 minutes  
**Content**: All settings, options, and troubleshooting

### 🔧 For Developers
**File**: [README.md](README.md)  
**Read Time**: 15-20 minutes  
**Content**: Technical details, architecture, API

### 🏗️ For System Architects
**File**: [ARCHITECTURE.md](ARCHITECTURE.md)  
**Read Time**: 15-20 minutes  
**Content**: System design, data flow, diagrams

### 📋 For Reference
**File**: [SUMMARY.md](SUMMARY.md)  
**Read Time**: 10 minutes  
**Content**: Overview, features, checklist

**File**: [FILES.md](FILES.md)  
**Read Time**: 5-10 minutes  
**Content**: File manifest, structure, descriptions

## 🎓 Reading Path

Choose based on your interest:

### "I just want to use it"
1. This file (INDEX.md) - now reading
2. [QUICKSTART.md](QUICKSTART.md) - 5 min
3. Run `bash install.sh` - 10 min
4. Add widget to panel - 1 min
✓ Total: 15 minutes to working widget

### "I want to understand it"
1. This file (INDEX.md) - now reading
2. [SUMMARY.md](SUMMARY.md) - 10 min
3. [QUICKSTART.md](QUICKSTART.md) - 5 min
4. Run `bash install.sh` - 10 min
5. [CONFIGURATION.md](CONFIGURATION.md) - 15 min for specific topics
✓ Total: 40 minutes for full understanding

### "I'm a developer/sysadmin"
1. [SUMMARY.md](SUMMARY.md) - 10 min overview
2. [README.md](README.md) - 20 min technical
3. [ARCHITECTURE.md](ARCHITECTURE.md) - 20 min design
4. Source code in `src/` and `package/`
5. Build with `bash install.sh`
✓ Total: 1 hour for complete understanding

## 📁 File Structure

```
kde-widget/
├── INDEX.md              ← You are here
├── QUICKSTART.md         ← 5 min quick setup
├── README.md             ← Full technical docs
├── CONFIGURATION.md      ← Settings guide
├── ARCHITECTURE.md       ← System design
├── SUMMARY.md            ← Overview
├── FILES.md              ← File manifest
│
├── install.sh            ← Run this (chmod +x already done)
├── CMakeLists.txt        ← Build configuration
│
├── src/                  ← C++ source code
│   ├── razercontrolwidget.h/cpp
│   ├── daemoncommunicator.h/cpp
│   └── resources.qrc
│
└── package/              ← KDE package
    ├── metadata.json
    └── contents/
        ├── ui/main.qml
        └── config/main.xml
```

## ✨ Key Capabilities

| Feature | Status | Details |
|---------|--------|---------|
| System Tray Widget | ✅ | Shows in bottom-right corner |
| Quick Control Menu | ✅ | Right-click for fast access |
| Auto-Start on Boot | ✅ | Automatic daemon launch |
| Start Minimized | ✅ | Hidden on boot option |
| Battery Monitoring | ✅ | Real-time % display |
| Fan Control | ✅ | Quick access from menu |
| Power Profiles | ✅ | Quick switch in menu |
| RGB Control | ✅ | Quick access from menu |
| Battery Health | ✅ | BHO settings from menu |
| Configuration Panel | ✅ | Full settings control |
| Daemon Communication | ✅ | JSON over local socket |
| Installation Script | ✅ | Automated with checks |

## 🛠️ Installation Methods

### Method 1: Automated (Recommended)
```bash
cd razer_control_gui/kde-widget
bash install.sh
```
✓ Automatic dependency checking  
✓ Automatic build configuration  
✓ Automatic installation  
✓ Auto-start setup  
✓ Time: 5-15 minutes (depends on system)

### Method 2: Manual (Advanced)
```bash
cd razer_control_gui/kde-widget
mkdir build && cd build
cmake ..
make
sudo make install
kbuildsycoca6  # Rebuild KDE cache
```

## ⚙️ Configuration

After installation, configure by:
1. Right-click the widget icon in system tray
2. Select "Configuration"
3. Adjust settings:
   - **Start Minimized**: Hide window on boot
   - **Auto-Start**: Launch on system startup
   - **Show Battery**: Display % in tooltip
   - **Refresh Rate**: Update frequency (1-10 sec)

Settings are saved automatically to `~/.config/plasmarc`

## 🔄 How It Works

```
BOOT
  ↓
Auto-start entry executes: razer-settings --minimized
  ↓
Daemon starts in background
  ↓
Widget appears in system tray
  ↓
USER INTERACTS
  ├─ Left-click → Full GUI opens
  ├─ Right-click → Quick menu appears
  └─ Hover → Battery % tooltip shows
  ↓
Actions sent to daemon → Hardware updated
  ↓
Widget updates display → User sees new status
```

## 🆘 Help & Support

### Common Issues

**Widget not showing?**
```bash
kbuildsycoca6
kquitapp6 plasmashell; sleep 2; kstart6 plasmashell &
```

**Daemon not connecting?**
```bash
systemctl status razercontrol
journalctl -u razercontrol
```

**Auto-start not working?**
```bash
cat ~/.config/autostart/razer-settings.desktop
chmod 644 ~/.config/autostart/razer-settings.desktop
```

### Full Troubleshooting
See [CONFIGURATION.md](CONFIGURATION.md) - Troubleshooting section

### Getting Help
1. Check relevant documentation above
2. View daemon logs: `journalctl -u razercontrol -n 50`
3. Verify files: `ls ~/.config/autostart/razer-settings.desktop`
4. Report issues: [GitHub Issues](https://github.com/encomjp/razer-control)

## 📊 System Requirements

- **KDE Plasma**: 5.27+ (or Plasma 6.x)
- **Qt**: 6.0+ (or 5.15+)
- **KDE Frameworks**: KF6 (or KF5)
- **Razer Control**: daemon and GUI installed
- **Linux**: Any modern distribution

**Build Requirements**:
- GCC/Clang with C++17 support
- CMake 3.16+
- Standard development tools

## 📞 Quick Links

| Link | Purpose |
|------|---------|
| [QUICKSTART.md](QUICKSTART.md) | 5-min setup guide |
| [README.md](README.md) | Full documentation |
| [CONFIGURATION.md](CONFIGURATION.md) | Settings reference |
| [ARCHITECTURE.md](ARCHITECTURE.md) | System design |
| [SUMMARY.md](SUMMARY.md) | Feature overview |
| [FILES.md](FILES.md) | File reference |
| `install.sh` | Installation script |

## ✅ Installation Checklist

- [ ] Run `bash install.sh`
- [ ] Confirm dependencies installed
- [ ] Wait for build and installation
- [ ] Choose to restart Plasma
- [ ] Right-click panel → Add Widgets
- [ ] Search for "Razer Control"
- [ ] Click to add widget
- [ ] Left-click to test (opens settings)
- [ ] Right-click to test (shows menu)
- [ ] Hover to test (tooltip appears)
- [ ] Reboot to verify auto-start

## 🎉 You're Ready!

Everything is set up. Next step:

```bash
cd razer_control_gui/kde-widget
bash install.sh
```

The script will guide you through the installation. Once done, your Razer laptop controls will be just a right-click away in the system tray!

## 📝 Notes

- Widget updates battery % every 2 seconds (configurable)
- Auto-start creates entry in `~/.config/autostart/`
- Configuration stored in `~/.config/plasmarc`
- Communicates with daemon via local socket
- All features accessible from right-click menu
- Full GUI still available by left-clicking

## 🚀 Next Steps

1. **Install**: Run the installation script
2. **Configure**: Customize settings if needed
3. **Use**: Click widget for settings, right-click for quick menu
4. **Enjoy**: Fast control of your Razer laptop!

---

**Version**: 0.2.0  
**License**: GPLv2+  
**Status**: Production Ready ✅  

Happy controlling! 🎮
