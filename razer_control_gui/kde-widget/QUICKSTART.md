# KDE Widget Quick Start

## Installation (Easy Method)

```bash
cd razer_control_gui/kde-widget
bash install.sh
```

The installer will:
✓ Verify dependencies  
✓ Build the widget  
✓ Install to system  
✓ Configure auto-start  
✓ Optionally restart Plasma  

## Using the Widget

### 1. Add Widget to Panel

1. Right-click on panel (bottom of screen)
2. Select "Edit Panel" or "Add Widgets"
3. Search for "Razer Control"
4. Click to add

The widget icon appears in your system tray (bottom-right).

### 2. Interact with Widget

**Left-click**: Open full Razer Settings application

**Right-click**: Access quick menu with options:
- Open Settings
- Fan Control
- Power Profiles  
- RGB Control
- Battery Health
- Minimize app
- Settings
- Exit

**Hover**: See tooltip with device name and battery %

### 3. Configure

Right-click widget → "Configuration"

Options:
- ✓ Start minimized on boot
- ✓ Auto-start on system boot
- ✓ Show battery % in widget
- Refresh interval (1-10 seconds)

## Key Features

🎯 **Bottom-Right System Tray**
- Always visible, quick access
- Shows battery percentage tooltip

⚙️ **Quick Menu**
- Fast access to common controls
- No need to open full window

🚀 **Auto-Start**
- Optionally auto-launch on boot
- Can start minimized to tray

🔄 **Live Updates**
- Battery % updates every 2 seconds (configurable)
- Device status always current

## File Structure

```
kde-widget/
├── install.sh              ← Run this to install
├── README.md               ← Full documentation
├── CONFIGURATION.md        ← Configuration guide
├── CMakeLists.txt          ← Build config
├── src/
│   ├── razercontrolwidget.cpp    ← Main widget logic
│   ├── daemoncommunicator.cpp    ← Daemon communication
│   └── resources.qrc
└── package/
    ├── metadata.json       ← Widget info
    └── contents/
        ├── ui/main.qml     ← Widget interface
        └── config/main.xml ← Settings schema
```

## Troubleshooting

**Widget not appearing?**
```bash
kbuildsycoca6  # Rebuild KDE cache (KDE 6)
# or kbuildsycoca5 for KDE 5
```

**Shows "Detecting..."?**
```bash
systemctl status razercontrol  # Check daemon is running
journalctl -u razercontrol     # View daemon logs
```

**Auto-start not working?**
```bash
cat ~/.config/autostart/razer-settings.desktop  # Check file exists
chmod 644 ~/.config/autostart/razer-settings.desktop  # Fix permissions
```

## Next Steps

1. Run: `bash install.sh`
2. Restart Plasma when prompted
3. Right-click your panel → Add Widgets → Search "Razer Control"
4. Right-click widget to access quick menu
5. Customize in widget settings if needed

## Full Documentation

- [Complete README](README.md) - Technical details and architecture
- [Configuration Guide](CONFIGURATION.md) - All settings and options
- [Razer Control Main Project](https://github.com/encomjp/razer-control)
