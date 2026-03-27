<div align="center">

# 🐍 Razer Laptop Control — Revived

### Take full control of your Razer Blade on Linux. No kernel modules. No DKMS. Just works.

[![License: GPL-2.0](https://img.shields.io/badge/License-GPL%202.0-blue.svg?style=flat-square)](LICENSE)
[![Release](https://img.shields.io/github/v/release/encomjp/razer-control-revived?style=flat-square&color=brightgreen)](https://github.com/encomjp/razer-control-revived/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/encomjp/razer-control-revived/total?style=flat-square&color=orange)](https://github.com/encomjp/razer-control-revived/releases)
[![Stars](https://img.shields.io/github/stars/encomjp/razer-control-revived?style=flat-square&color=yellow)](https://github.com/encomjp/razer-control-revived/stargazers)

Fan curves · Power profiles · CPU/GPU boost · Battery health · RGB effects · System tray — all in one place.

---

<a href="https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A"><img src="https://img.shields.io/badge/%E2%98%95_Buy_Me_a_Coffee-PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white" alt="Donate" height="36" /></a>

---

</div>

## 🖼️ Screenshots

<div align="center">

<img alt="Main Window - Overview" src="https://github.com/user-attachments/assets/48b10737-76ed-45a0-886b-df5ba6bea30d" width="80%" />

*GTK4 / libadwaita GUI — AC & Battery profiles with live system stats*

</div>

<table>
<tr>
<td width="50%" align="center">
<img alt="Power Profile Tab" src="https://github.com/user-attachments/assets/7de72d59-7323-4933-a742-23c1100d63dd" />
<br><sub><b>⚡ Power Profiles & Fan Control</b></sub>
</td>
<td width="50%" align="center">
<img alt="RGB & Keyboard Tab" src="https://github.com/user-attachments/assets/cd1c5f19-02f7-4d1f-a570-4b90771ae6d7" />
<br><sub><b>🌈 Keyboard RGB & Effects</b></sub>
</td>
</tr>
<tr>
<td colspan="2" align="center">
<img alt="System Tray" src="https://github.com/user-attachments/assets/84b4fba5-6b77-4e13-aee5-5ac5f0b2b631" width="40%" />
<br><sub><b>🔔 System Tray with Sensor Tooltip</b></sub>
</td>
</tr>
</table>

---

> **⚠️ DISCLAIMER:** This is experimental community software. Use at your own risk. No warranty is provided.

---

## 📥 Download & Install

<div align="center">

### Pick your distro and get started in seconds

<br>

<a href="https://github.com/encomjp/razer-control-revived/releases/latest"><img src="https://img.shields.io/badge/🟠_Ubuntu_/_Debian-.deb_Package-E95420?style=for-the-badge&logoColor=white" alt="Download .deb" height="48" /></a>
&nbsp;&nbsp;
<a href="https://github.com/encomjp/razer-control-revived/releases/latest"><img src="https://img.shields.io/badge/🔵_Fedora_/_RHEL-.rpm_Package-51A2DA?style=for-the-badge&logoColor=white" alt="Download .rpm" height="48" /></a>
&nbsp;&nbsp;
<a href="https://github.com/encomjp/razer-control-revived/releases/latest"><img src="https://img.shields.io/badge/�_Any_Distro-Tarball-888888?style=for-the-badge&logoColor=white" alt="Download Tarball" height="48" /></a>

<br><br>

</div>

<details open>
<summary><h3>🟠 Ubuntu / Debian</h3></summary>

```bash
# Download the .deb from the releases page, then:
sudo apt install ./razercontrol-revived_0.3.0-rc3_amd64.deb
```
Installs everything: daemon, CLI, GUI, systemd service, udev rules, and desktop entry.
</details>

<details>
<summary><h3>🔵 Fedora / RHEL</h3></summary>

```bash
# Download the .rpm from the releases page, then:
sudo dnf install ./razercontrol-0.3.0-rc3-1.fc41.x86_64.rpm
```
Installs everything: daemon, CLI, GUI, systemd service, udev rules, and desktop entry.
</details>

<details>
<summary><h3>� Tarball (Any Distribution)</h3></summary>

```bash
# Download the tarball from the releases page, then:
tar -xzf razer-control-0.3.0-rc3-x86_64.tar.gz
cd razer-control-0.3.0-rc3-x86_64
sudo ./install.sh
```
Installs everything: daemon, CLI, GUI, systemd service, udev rules, and desktop entry.
</details>

<details>
<summary><h3>❄️ NixOS</h3></summary>

Add to your flake inputs:
```nix
inputs.razerdaemon = {
  url = "github:encomjp/razer-control-revived";
  inputs.nixpkgs.follows = "nixpkgs";
};
```
Import and enable:
```nix
imports = [ inputs.razerdaemon.nixosModules.default ];
services.razer-laptop-control.enable = true;
```
</details>

<details>
<summary><h3>🔨 Arch Linux / Build from Source</h3></summary>

```bash
# Install dependencies (Arch example)
sudo pacman -S rust cargo dbus libusb hidapi pkgconf systemd gtk4 libadwaita git

# Clone and install
git clone https://github.com/encomjp/razercontrol-revived
cd razercontrol-revived/razer_control_gui
./install.sh install
```
</details>

> **📝 Note:** Log out and back in (or reboot) after installation for udev rules to take effect.

---

## ✨ Features

| | Feature | Description |
|---|---|---|
| 🌀 | **Fan Control** | Auto mode or manual RPM (2200–5000+ depending on model) |
| ⚡ | **Power Profiles** | Balanced, Gaming, Creator, Silent, or Custom |
| 🚀 | **CPU/GPU Boost** | Fine-tune performance — Low / Normal / High / Boost |
| 💡 | **Logo LED** | Off, On, or Breathing modes |
| 🌈 | **Keyboard RGB** | Brightness + effects: Static, Wave, Breathing, Spectrum, Reactive |
| 🔋 | **Battery Health (BHO)** | Limit charge to 50–80% to extend battery lifespan |
| 📊 | **System Monitor** | Live CPU/iGPU/dGPU temps, power draw, utilization, battery |
| 🔔 | **System Tray** | KDE tray icon with sensor tooltip, close-to-tray |
| 🖥️ | **GTK4 GUI** | Modern libadwaita interface with separate AC/Battery profiles |
| ⌨️ | **CLI** | Full command-line control for scripting & automation |
| 🔄 | **Daemon** | Auto-loads your saved settings on startup |

---

## 📋 Supported Devices

> **Works with 50+ Razer Blade laptops** — from 2015 Stealth to 2025 Blade 16.

<details>
<summary><b>Click to expand full device list</b></summary>

| Model | Year | USB PID | Status |
|-------|------|---------|--------|
| Blade Stealth | 2015–2020 | Various | ✅ Supported |
| Blade 15 | 2016–2023 | Various | ✅ Supported |
| Blade Pro | 2017–2021 | Various | ✅ Supported |
| Blade 14 | 2021–2025 | Various | ✅ Supported |
| Blade 16 | 2023–2025 | Various | ✅ Supported |
| Blade 17 | 2022 | 028B | ✅ Supported |
| Blade 18 | 2023–2025 | Various | ✅ Supported |
| Razer Book 13 | 2020 | 026A | ✅ Supported |
| **Blade 16 2025** | 2025 | **02C6** | ✅ **Tested** |

</details>

**Check if your laptop is supported:**
```bash
lsusb | grep -i razer
# Look for: Bus XXX Device XXX: ID 1532:XXXX Razer USA, Ltd
# The XXXX after 1532: is your device's USB PID
```

---

## 🛠️ Build Dependencies

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

---

## 🧩 KDE Plasma Widget

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

---

## 🚀 Usage

### GUI Application

Launch from your application menu (search "Razer Settings") or run:
```bash
razer-settings
```

The GUI provides separate tabs for AC and Battery power profiles, allowing different settings for each.

<details>
<summary><h3>⌨️ Command Line Interface</h3></summary>

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
</details>

<details>
<summary><h3>🌈 RGB Effects</h3></summary>

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
</details>

<details>
<summary><h3>🔄 Service Management</h3></summary>

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
</details>

---

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

---

## 🗑️ Uninstallation

```bash
cd razercontrol-revived/razer_control_gui
./install.sh uninstall
```

---

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

---

## ⚠️ Warning

This software is provided AS-IS with **NO WARRANTY**.

| | |
|---|---|
| ❌ | Not affiliated with Razer Inc. |
| ❌ | Not responsible for any damage to your hardware |
| ❌ | No official support — community project only |
| ✅ | Works on my machine™ (Blade 16 2025 / RTX 5070 Ti) |

---

## 🙏 Credits

**Core**
- **Original project:** [Razer-Linux/razer-laptop-control-no-dkms](https://github.com/Razer-Linux/razer-laptop-control-no-dkms)
- **HID modifications, GTK4 implementation & packaging:** [@encomjp](https://github.com/encomjp)
- **UI rework (native libadwaita widgets, CSS cleanup):** [Claude](https://claude.ai/) by Anthropic

**Contributors**
| Who | What |
|-----|------|
| [@johva1312](https://github.com/johva1312) | HID device init fallbacks — prefer `iface-0`, use `hidraw` as fallback for broader device compatibility; Fix partial socket reads — replace fixed-size `read()` with `read_to_end()` to prevent unexpected EOF (PR #8) |
| [@sini](https://github.com/sini) | NixOS flake fixes — updated nixpkgs, fixed typos, ensured version parity |

---

<div align="center">

## 📄 License

This project is licensed under the **GPL-2.0** license — see the [LICENSE](LICENSE) file for details.

<br>

<a href="https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A"><img src="https://img.shields.io/badge/%E2%98%95_Support_Development-PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white" alt="Donate" height="36" /></a>

<br><br>

**⭐ If this project helps you, give it a star!**

</div>
