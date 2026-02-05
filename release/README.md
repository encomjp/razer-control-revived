# Razer Control - Release Packages

Pre-built packages for Razer Control v0.2.0.

## Quick Install Guide

| Distribution | Recommended Package |
|--------------|---------------------|
| **Fedora / RHEL / CentOS** | RPM package |
| **Ubuntu / Debian / Arch / Others** | AppImage (universal) |

---

## Option 1: RPM Package (Fedora/RHEL) ✅ Recommended for Fedora

**Best for:** Fedora, RHEL, CentOS, openSUSE, and other RPM-based distributions.

```bash
sudo dnf install ./razercontrol-0.2.0-1.fc43.x86_64.rpm
```

This installs everything: daemon, CLI, GUI, systemd service, udev rules, and desktop entry.

**After installation:** Log out and back in (or reboot) for udev rules to take effect.

---

## Option 2: AppImage (Universal) ✅ Works on Any Linux

**Best for:** Any Linux distribution, or if the RPM doesn't work for your system.

The AppImage is a self-contained portable application that works on virtually any Linux distribution.

### Full Installation (Recommended)

1. **Download and extract the tarball** to install the daemon and services:
   ```bash
   tar -xzf razer-control-0.2.0-x86_64.tar.gz
   cd razer-control-0.2.0-x86_64
   sudo ./install.sh
   ```

2. **Run the AppImage** for the GUI:
   ```bash
   chmod +x RazerControl-0.2.0-x86_64.AppImage
   ./RazerControl-0.2.0-x86_64.AppImage
   ```

### Why Both?

The AppImage contains only the GUI application. The **daemon** (background service that communicates with your Razer hardware) must be installed system-wide because it requires:
- Access to `/dev/hidraw*` devices (USB communication)
- udev rules for device permissions
- systemd service for auto-start

---

## Package Contents

| File | Size | Description |
|------|------|-------------|
| `razercontrol-0.2.0-1.fc43.x86_64.rpm` | 1.3 MB | Complete RPM package (Fedora/RHEL) |
| `RazerControl-0.2.0-x86_64.AppImage` | 2.7 MB | Portable GUI (universal, needs daemon) |
| `razer-control-0.2.0-x86_64.tar.gz` | 2.0 MB | Tarball with install script |

## Tarball Contents

```
razer-control-0.2.0-x86_64/
├── bin/
│   ├── daemon          # Background daemon (required)
│   ├── razer-cli       # Command-line interface
│   └── razer-settings  # GTK4 GUI application
├── share/
│   ├── applications/
│   │   └── razer-settings.desktop
│   └── razercontrol/
│       └── laptops.json
├── systemd/
│   └── razercontrol.service
├── udev/
│   └── 99-hidraw-permissions.rules
├── kde-widget/         # KDE Plasma 6 widget
│   ├── metadata.json
│   └── contents/ui/main.qml
└── install.sh          # Installation script (run as root)
```

## KDE Plasma 6 Widget

The tarball includes the KDE Plasma 6 widget. To install:

```bash
# From extracted tarball directory
mkdir -p ~/.local/share/plasma/plasmoids/com.github.encomjp.razercontrol
cp -r kde-widget/* ~/.local/share/plasma/plasmoids/com.github.encomjp.razercontrol/
kbuildsycoca6
```

Then add to your panel: Right-click panel → Add Widgets → Search "Razer Control"

---

## Troubleshooting

### Device not detected
- Make sure udev rules are installed and you've logged out/in
- Check that the daemon is running: `systemctl status razercontrol`

### Permission denied
- Run `sudo udevadm control --reload-rules && sudo udevadm trigger`
- Log out and back in

### AppImage won't start
- Make sure it's executable: `chmod +x RazerControl-*.AppImage`
- Install the daemon first using the tarball
