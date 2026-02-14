# Razer Laptop Control - Revived

<img width="525" height="439" alt="image" src="https://github.com/user-attachments/assets/84b4fba5-6b77-4e13-aee5-5ac5f0b2b631" />


<img width="1059" height="870" alt="image" src="https://github.com/user-attachments/assets/48b10737-76ed-45a0-886b-df5ba6bea30d" />

<img width="1069" height="871" alt="image" src="https://github.com/user-attachments/assets/7de72d59-7323-4933-a742-23c1100d63dd" />

<img width="1068" height="877" alt="image" src="https://github.com/user-attachments/assets/cd1c5f19-02f7-4d1f-a570-4b90771ae6d7" />


[![License: GPL-2.0](https://img.shields.io/badge/License-GPL%202.0-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/encomjp/razer-control-revived)](https://github.com/encomjp/razer-control-revived/releases/latest)
[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A)

A Linux userspace application to control Razer Blade laptops. **No kernel modules (DKMS) required!**

This tool provides more control over your Razer laptop than Synapse does - fan curves, power profiles, CPU/GPU boost, battery health optimization, and RGB effects all in one place.

> **⚠️ DISCLAIMER:** This is experimental community software. Use at your own risk. No warranty is provided.

## ☕ Support This Project

If you find this project useful, please consider supporting its development:

[![Donate with PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?style=for-the-badge&logo=paypal)](https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A)

## 📥 Downloads

**[⬇️ Download Latest Release (v0.2.8)](https://github.com/encomjp/razer-control-revived/releases/tag/v0.2.8)**

| Package | Best For | Description |
|---------|----------|-------------|
| `razercontrol-revived_*.deb` | **Ubuntu / Debian** | Complete .deb package - installs everything |
| `razercontrol-*.rpm` | **Fedora / RHEL** | Complete RPM package - installs everything |
| `RazerControl-*.AppImage` | **All distros** | Universal portable GUI (needs daemon) |
| `razer-control-*.tar.gz` | **Manual install** | Tarball with install script |

### Ubuntu / Debian
```bash
sudo apt install ./razercontrol-revived_0.2.8_amd64.deb
```

### Fedora / RHEL
```bash
sudo dnf install ./razercontrol-0.2.8-1.fc41.x86_64.rpm
```

### All Other Distributions (AppImage)

Install the daemon first, then use the portable AppImage for the GUI:

```bash
# 1. Install daemon from tarball
tar -xzf razer-control-0.2.8-x86_64.tar.gz
cd razer-control-0.2.8-x86_64
sudo ./install.sh

# 2. Run the AppImage
chmod +x RazerControl-0.2.8-x86_64.AppImage
./RazerControl-0.2.8-x86_64.AppImage
```

> **Note:** Log out and back in (or reboot) after installation for udev rules to take effect.

## ✨ Features

- 🌀 **Fan Control** - Auto mode or manual RPM (2200-5000+ depending on model)
- ⚡ **Power Profiles** - Balanced, Gaming, Creator, Silent, or Custom
- 🚀 **CPU/GPU Boost** - Fine-tune performance (Low/Normal/High/Boost)
- 💡 **Logo LED** - Off, On, or Breathing
- 🌈 **Keyboard RGB** - Brightness control and effects (Static, Wave, Breathing, Spectrum, etc.)
- 🔋 **Battery Health Optimizer (BHO)** - Limit charge to 50-80% to extend battery lifespan
- 📊 **System Monitor** - Live CPU/iGPU/dGPU temps, power draw, utilization, battery status
- 🔔 **System Tray** - KDE system tray icon with sensor tooltip, close-to-tray
- 🖥️ **GTK4 GUI** - Modern libadwaita interface with separate AC/Battery profiles
- ⌨️ **CLI** - Full command-line control for scripting
- 🔄 **Daemon** - Auto-loads your settings on startup

## 📋 Supported Devices

| Model | Year | USB PID | Status |
|-------|------|---------|--------|
| Blade Stealth | 2015-2020 | Various | ✅ Supported |
| Blade 15 | 2016-2023 | Various | ✅ Supported |
| Blade Pro | 2017-2021 | Various | ✅ Supported |
| Blade 14 | 2021-2025 | Various | ✅ Supported |
| Blade 16 | 2023-2025 | Various | ✅ Supported |
| Blade 17 | 2022 | 028B | ✅ Supported |
| Blade 18 | 2023-2025 | Various | ✅ Supported |
| Razer Book 13 | 2020 | 026A | ✅ Supported |
| **Blade 16 2025** | 2025 | **02C6** | ✅ **Tested** |

**Check if your laptop is supported:**
```bash
lsusb | grep -i razer
# Look for: Bus XXX Device XXX: ID 1532:XXXX Razer USA, Ltd
# The XXXX after 1532: is your device's USB PID
```

## 📦 Installation

### Dependencies

<details>
<summary><b>Fedora / RHEL / CentOS</b></summary>

```bash
sudo dnf install -y rust cargo dbus-devel libusb1-devel hidapi-devel \
    pkgconf systemd-devel gtk4-devel libadwaita-devel git
```
</details>

<details>
<summary><b>Ubuntu / Debian</b></summary>

```bash
sudo apt install -y rustc cargo libdbus-1-dev libusb-1.0-0-dev libhidapi-dev \
    pkg-config libsystemd-dev libgtk-4-dev libadwaita-1-dev git
```
</details>

<details>
<summary><b>Arch Linux</b></summary>

```bash
sudo pacman -S rust cargo dbus libusb hidapi pkgconf systemd gtk4 libadwaita git
```
</details>

<details>
<summary><b>NixOS</b></summary>

Add to your flake inputs:
```nix
inputs.razerdaemon = {
  url = "github:encomjp/razer-control-revived";
  inputs.nixpkgs.follows = "nixpkgs";
};
```

Import the module:
```nix
imports = [ inputs.razerdaemon.nixosModules.default ];
```

Enable the service:
```nix
services.razer-laptop-control.enable = true;
```
</details>

### Build & Install

```bash
# Clone the repository
git clone https://github.com/encomjp/razercontrol-revived.git
cd razercontrol-revived/razer_control_gui

# Build and install (will prompt for sudo)
./install.sh install
```

After installation, **log out and back in** (or reboot) for udev rules to take effect.

## �️ KDE Plasma Widget

A native KDE Plasma 6 widget is available for quick access from your panel.

### Install the Widget

```bash
cd razer_control_gui/kde-widget
./install-plasmoid.sh
```

Then add it to your desktop or panel: Right-click → Add Widgets → Search "Razer Control"

The widget shows:
- **Live system monitor** - CPU/iGPU/dGPU temps, frequencies, power draw (including CPU package via RAPL), and utilization
- **Battery status** - charge %, charging/discharging wattage with progress bar
- **Clickable settings** - Profile, Fan, KB Brightness, Logo, Charge Limit (click to cycle) in a unified grouped card
- **Correct iGPU naming** - Properly detects Radeon 880M (AI 365) vs 890M (AI 370) from CPU model

See [kde-widget/README.md](razer_control_gui/kde-widget/README.md) for more details.

## �🚀 Usage

### GUI Application

Launch from your application menu (search "Razer Settings") or run:
```bash
razer-settings
```

The GUI provides separate tabs for AC and Battery power profiles, allowing different settings for each.

### Command Line Interface

```bash
# Get help
razer-cli --help
razer-cli read --help
razer-cli write --help

# Read current settings (use 'ac' for plugged in, 'bat' for battery)
razer-cli read fan ac           # Fan speed
razer-cli read power ac         # Power profile
razer-cli read brightness ac    # Keyboard brightness
razer-cli read logo ac          # Logo LED state
razer-cli read bho              # Battery Health Optimizer

# Fan control (0 = auto, or specify RPM)
razer-cli write fan ac 0        # Auto
razer-cli write fan ac 4000     # 4000 RPM

# Power modes: 0=Balanced, 1=Gaming, 2=Creator, 3=Silent, 4=Custom
razer-cli write power ac 1 0 0  # Gaming mode (basic)
razer-cli write power ac 4 2 2  # Custom with CPU=High, GPU=High

# Keyboard brightness (0-100)
razer-cli write brightness ac 75

# Logo LED: 0=Off, 1=On, 2=Breathing
razer-cli write logo ac 1

# Battery Health Optimizer (limit charge %)
razer-cli write bho on 80       # Limit to 80%
razer-cli write bho off         # Disable limit
```

### RGB Effects

```bash
# Static color
razer-cli standard-effect static 0 255 0      # Green

# Wave effect (direction: 1=left, 2=right)
razer-cli standard-effect wave 1

# Breathing (type: 1=single, 2=dual, 3=random)
razer-cli standard-effect breathing 1 255 0 0  # Red breathing

# Spectrum cycle
razer-cli standard-effect spectrum

# Reactive (speed 1-4, then R G B)
razer-cli standard-effect reactive 2 255 255 0

# Turn off
razer-cli standard-effect off
```

### Service Management

The daemon runs as a **systemd user service** (no root required):

```bash
# Check daemon status
systemctl --user status razercontrol

# Restart daemon
systemctl --user restart razercontrol

# View logs
journalctl --user -u razercontrol -f

# Enable/disable auto-start
systemctl --user enable razercontrol
systemctl --user disable razercontrol
```

## 🔧 Troubleshooting

<details>
<summary><b>"No supported device found"</b></summary>

Your laptop's USB PID might not be in the device list.

1. Find your PID:
   ```bash
   lsusb | grep -i razer
   ```

2. Add it to the device list:
   ```bash
   sudo nano /usr/share/razercontrol/laptops.json
   ```

3. Restart the daemon:
   ```bash
   systemctl --user restart razercontrol
   ```

See [Adding Support for New Devices](#-adding-support-for-new-devices) for details.
</details>

<details>
<summary><b>"Permission denied" on hidraw</b></summary>

The udev rules might not have been applied:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Then log out and back in, or reboot.
</details>

<details>
<summary><b>Daemon not starting / Socket doesn't exist</b></summary>

Check if another Razer service is conflicting:
```bash
systemctl --user list-units | grep -i razer
```

Disable any conflicting services:
```bash
sudo systemctl stop razer-service
sudo systemctl disable razer-service
```
</details>

<details>
<summary><b>GUI shows "Cannot connect to daemon"</b></summary>

1. Check if the daemon is running:
   ```bash
   systemctl --user status razercontrol
   ```

2. If not running, start it:
   ```bash
   systemctl --user start razercontrol
   ```

3. Check logs for errors:
   ```bash
   journalctl --user -u razercontrol -n 50
   ```
</details>

## 🗑️ Uninstallation

```bash
cd razercontrol-revived/razer_control_gui
./install.sh uninstall
```

## ➕ Adding Support for New Devices

If your Razer laptop isn't supported, you can add it:

1. **Find your device's USB PID:**
   ```bash
   lsusb | grep -i razer
   # Example output: ID 1532:02c6 Razer USA, Ltd
   # Your PID is: 02c6
   ```

2. **Edit the device list** (`data/devices/laptops.json`):
   ```json
   {
       "name": "Blade XX 20XX",
       "vid": "1532",
       "pid": "YOUR_PID_HERE",
       "features": ["logo", "boost", "bho"],
       "fan": [2200, 5000]
   }
   ```

3. **Add your PID to udev rules** (`data/udev/99-hidraw-permissions.rules`):
   ```
   KERNEL=="hidraw*", ATTRS{idVendor}=="1532", ATTRS{idProduct}=="YOUR_PID", MODE="0666"
   ```

4. **Reinstall:**
   ```bash
   ./install.sh install
   ```

5. **Submit a PR!** Help others with the same laptop.

## ⚠️ Warning

This software is provided AS-IS with **NO WARRANTY**.

- ❌ Not affiliated with Razer Inc.
- ❌ Not responsible for any damage to your hardware
- ❌ No official support - community project only
- ✅ Works on my machine™ (Blade 16 2025 with RTX 5070 Ti)

## 🙏 Credits

- Original project: [Razer-Linux/razer-laptop-control-no-dkms](https://github.com/Razer-Linux/razer-laptop-control-no-dkms)
- HID modifications, GTK4 implementation & package porting: [@encomjp](https://github.com/encomjp)
- UI rework (native libadwaita widgets, CSS cleanup): [Claude](https://claude.ai/) by Anthropic

## 📄 License

GPL-2.0 - See [LICENSE](LICENSE) file
