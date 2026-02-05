# Razer Laptop Control GUI & CLI

## ❤️ Support This Project

If you find this project useful, please consider donating to support continued development and add support for more Razer blade models:

[![Donate](https://img.shields.io/badge/Donate-PayPal-blue.svg)](https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A)

Your contribution helps me:
- 🔧 Add support for more Razer laptop models
- 🐛 Fix bugs and improve stability
- 📱 Implement new features
- 📚 Improve documentation

## ⚠️ Important Disclaimer

**Tested on:** Fedora Linux (as of February 2026)

This project has been primarily tested on Fedora. It should work on Ubuntu and similar Linux distributions, but **no guarantees are given**. If you experience issues on other distributions:
1. Check the [Issues](https://github.com/encomjp/razer-control/issues) page
2. Open a new issue with your distribution and error details
3. I will work with you to add support

**⚠️ WARNING:** Incorrectly configuring hardware controls (especially fan speeds) could potentially damage your device. Use at your own risk and always monitor your system temperatures.

## Current features

- Full background daemon - Auto load state on machine startup based on last configuration
- CLI and GUI application for adjusting settings
- Battery Health Optimizer (BHO) support
- Supports laptops from 2016-2025 including the new Blade 2025

![](Screenshoot.png)

## Installing

### Compiling from source

1. Install cargo or rustc
2. add `libdbus-1-dev libusb-dev libhidapi-dev libhidapi-hidraw0 pkg-config libudev-dev libgtk-3-dev` packages (or equivelent)
3. run `./install.sh install` as a normal user
4. reboot
5. Enjoy!

### Nixos flake installation

1. Add this flake to your inputs using

```
inputs.razerdaemon.url = "github:JosuGZ/razer-laptop-control";
```

2. Import the razerdaemon module where your inputs are in scope

```
imports = [
    inputs.razerdaemon.nixosModules.default
];
```

3. Enable the exposed nixos option using

```
services.razer-laptop-control.enable = true;
```

## Usage of CLI Application

```
razer-cli <action> <attribute> <power_state> <args>
```

### action

- read - Read an attribute (get its current state) - No additional args are supplied
- write - Write an attribute, and save it to configuration - See below for argument counts

### attribute

- fan - Fan RPM. ARG: 0 = Auto, anything else is interpreted as a litteral RPM
- power - Power mode. ARG: 0 = Balanced, 1 = Gaming, 2 = Creator, 4 = Custom
- brightness - Change brightness of the keyboard
- logo - change logo state (for models with logo): 0 = off, 1 = on, 2 = breathing
- sync - sync light effect for battery/ac
- bho - Battery Health Optimizer. ARG: on/off [threshold 50-80]
- standard_effect - effects predefined in keyboard controller
- colour - Keyboard colour. ARGS: R G B channels, each channel is set from 0 to 255

### power_state

- ac
- bat

#### standard_effects

- 'off'
- 'wave' - PARAMS: <Direction>
- 'reactive' - PARAMS: <Speed> <Red> <Green> <Blue>
- 'breathing' - PARAMS: <Type> [Red] [Green] [Blue] [Red] [Green] [Blue]
- 'spectrum'
- 'static' - PARAMS: <Red> <Green> <Blue>
- 'starlight' - PARAMS: <Type> [Red] [Green] [Blue] [Red] [Green] [Blue]

#### custom power control

Custom power control take two more parameters: cpu boost and gpu boost

- 0 - low power
- 1 - normal
- 2 - high
- 3 - boost (only for CPU and only for Advanced 2020 model and Studio Edition)

```
razer-cli write power ac 4 3 2
```

## Warning

This software is provided AS-IS with NO WARRANTY. This is an experimental community project.

- I am NOT affiliated with Razer
- **I am not responsible for potential bricking etc.**
- No official support is provided - I'll try to help but no guarantees
- If something breaks, you get to keep both pieces

It worked on my AD AI 365 Razer Blade with 5070Ti and should potentially work on other 16 inch ones from 50xx series nvidia but no warranty.
