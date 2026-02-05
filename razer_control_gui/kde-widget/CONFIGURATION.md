# KDE Widget Configuration Guide

## Initial Setup

### 1. Installation

```bash
cd razer_control_gui/kde-widget
bash install.sh
```

The script will:
- Check for required dependencies
- Build the widget from source
- Install it to the KDE installation prefix
- Create auto-start configuration
- Optionally restart Plasma

### 2. Adding the Widget to Your Panel

1. **Right-click on your Plasma panel** (bottom of screen by default)
2. Select **"Edit Panel..."** or **"Add Widgets..."**
3. Click the **"+"** button to add a new widget
4. Search for **"Razer Control"**
5. Click it to add to your panel

The widget will appear as an icon in your system tray area.

## Using the Widget

### Widget Icon

The widget shows as a power management icon. The tooltip displays:
- "Razer Control"
- Current battery percentage
- Quick tip: "Click to open settings"

### Left-Click Actions

- **Click the widget icon**: Opens full Razer Settings application
- **Hold over the icon**: Shows tooltip with battery info

### Right-Click Context Menu

The context menu provides quick access to:

| Option | Action |
|--------|--------|
| Open Settings | Launch full Razer Settings GUI |
| Fan Control | Quick access to fan settings |
| Power Profiles | Switch power profiles (Gaming, Balanced, etc.) |
| RGB Control | Keyboard RGB brightness/effects |
| Battery Health | Battery Health Optimizer (BHO) settings |
| Minimize | Minimize/hide the main application window |
| Configuration | Open widget settings panel |
| Exit | Close the application |

## Configuration Options

### Accessing Widget Settings

**Method 1: Via Widget**
- Right-click widget icon → "Configuration"

**Method 2: Via System Settings**
- System Settings → Startup and Shutdown → Desktop Session → choose Razer Control

### Available Settings

#### General Tab

1. **Start application minimized on boot** (Default: Enabled)
   - When enabled, Razer Settings launches hidden in system tray
   - When disabled, full window opens on startup

2. **Auto-start on system boot** (Default: Enabled)
   - When enabled, automatically starts daemon and GUI at login
   - Creates entry in `~/.config/autostart/razer-settings.desktop`

3. **Show battery percentage in widget** (Default: Enabled)
   - Displays battery % in widget tooltip
   - Also shown in expanded widget view

4. **Refresh interval** (Default: 2 seconds, Range: 1-10)
   - How often widget updates battery/device info
   - Lower values = more responsive but higher CPU usage
   - Recommended: 2-3 seconds

## Auto-Start Configuration

The widget automatically manages auto-start through desktop entries.

### Manual Configuration

If you need to manually edit auto-start settings:

```bash
# Edit the auto-start entry
nano ~/.config/autostart/razer-settings.desktop
```

Standard desktop entry format:
```ini
[Desktop Entry]
Type=Application
Name=Razer Control
Comment=Control your Razer laptop settings
Exec=razer-settings --minimized
Icon=preferences-system-power-management
Categories=Utility;System;
Terminal=false
X-KDE-AutostartPhase=2
X-GNOME-Autostart-enabled=true
```

### Start Minimized Flag

To start the application minimized (in system tray):
```
Exec=razer-settings --minimized
```

To start with full window:
```
Exec=razer-settings
```

### Disable Auto-Start

Remove the auto-start file:
```bash
rm ~/.config/autostart/razer-settings.desktop
```

Or use System Settings:
- System Settings → Startup and Shutdown → Desktop Session
- Uncheck Razer Control

## Panel Positioning

### Bottom Panel (Default)

The widget is designed for the system tray at the bottom-right:

```
┌─ Window Title ───────────────────────────┐
│                                          │
│            Your Applications             │
│                                          │
├──────────────────────────────────────────┤
  [Other icons]  [Clock] [Razer Icon] ✕
```

### Other Panel Locations

The widget also works on:
- **Right panel**: Vertical orientation
- **Left panel**: Vertical orientation  
- **Top panel**: Horizontal orientation (less common)

To move your panel:
1. Right-click panel → "Edit Panel..."
2. Select "Panel Settings"
3. Choose desired position/orientation

## Integration with Razer Control

The widget communicates with the Razer Control daemon via local socket:
- **Socket location**: `~/.local/share/razer-daemon.sock`
- **Protocol**: JSON-based commands
- **Refresh rate**: Configurable (1-10 seconds)

The daemon must be running for full functionality. If the widget can't connect:
1. Check if daemon is running: `systemctl status razercontrol`
2. Restart daemon: `systemctl restart razercontrol`
3. Check daemon logs: `journalctl -u razercontrol -n 50`

## Troubleshooting

### Widget doesn't appear in add-widgets list

```bash
# Rebuild KDE configuration cache (KDE 6)
kbuildsycoca6

# Or for KDE 5
kbuildsycoca5

# Then restart Plasma
kquitapp6 plasmashell; sleep 2; kstart6 plasmashell &
# Or for KDE 5:
# kquitapp5 plasmashell; sleep 2; plasmashell &
```

### Widget shows "Detecting..." for device

1. Check daemon is running: `pgrep daemon`
2. Verify daemon socket exists: `ls -la ~/.local/share/razer-daemon.sock`
3. Check daemon logs for errors: `journalctl -u razercontrol`
4. Try restarting daemon: `systemctl restart razercontrol`

### Auto-start not working

1. Verify desktop entry exists:
   ```bash
   cat ~/.config/autostart/razer-settings.desktop
   ```

2. Check permissions:
   ```bash
   chmod 644 ~/.config/autostart/razer-settings.desktop
   ```

3. Ensure entry is enabled in System Settings:
   - System Settings → Startup and Shutdown → Desktop Session

4. Check for conflicts:
   ```bash
   ls ~/.config/autostart/ | grep -i razer
   ```

### High CPU usage

- Reduce refresh interval in widget settings (currently: check settings)
- Close the full Razer Settings window when not needed
- Check if daemon has runaway processes: `top -p $(pgrep daemon)`

### Widget menu doesn't respond

Try these steps:
1. Right-click widget → "Configure..."
2. Close and re-open any open Razer Settings windows
3. Restart Plasma: `kquitapp6 plasmashell & sleep 2 & kstart6 plasmashell &`

## Advanced: Widget Customization

### Changing the Widget Icon

Edit `package/metadata.json`:
```json
"Icon": "your-icon-name"
```

Available icon names: `qt5ct`, `kvantum`, `color-picker`, etc.

### Modifying Refresh Interval Range

Edit `package/contents/config/main.xml`:
```xml
<entry name="RefreshInterval" type="Int">
    <label>Refresh interval in seconds</label>
    <default>2</default>
    <min>1</min>
    <max>20</max>  <!-- Change max value here -->
</entry>
```

Then rebuild:
```bash
cd kde-widget/build
cmake .. && make && sudo make install
```

## Support

For issues or feature requests:
1. Check daemon logs: `journalctl -u razercontrol -f`
2. Verify Razer Control installation
3. Report issues to [Razer Control GitHub](https://github.com/encomjp/razer-control)

## See Also

- [KDE Plasma Documentation](https://userbase.kde.org/Plasma)
- [Razer Control Main README](../README.md)
- Widget source code: `kde-widget/src/` and `kde-widget/package/`
