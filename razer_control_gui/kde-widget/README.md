# Razer Control KDE Widget

A KDE Plasma 6 applet (widget) for controlling Razer laptops from the system tray. Provides quick access to Razer Settings with a simple left-click, and a context menu with donation link on right-click.

## ❤️ Support This Project

If you find this project useful, please consider donating to support continued development and add support for more Razer blade models:

[![Donate](https://img.shields.io/badge/Donate-PayPal-blue.svg)](https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A)

## ✅ Plasma 6 Compatibility

This widget is **Plasma 6 native** and has been tested on:
- **Fedora 43 KDE** (Plasma 6, KF6 6.22.0, Qt 6.10)

### Plasma 6 Technical Notes

The widget uses:
- `org.kde.plasma.plasma5support` for DataSource/process execution (not deprecated `org.kde.plasma.core`)
- `X-Plasma-API-Minimum-Version: 6.0` in metadata.json
- `KPackageStructure: Plasma/Applet` (new Plasma 6 format, replaces old `ServiceTypes`)
- Pure QML implementation (no C++ compilation required)

> **Note:** The C++ applet approach was abandoned due to significant Plasma 6 API changes. This QML-only plasmoid is simpler and more portable.

## Features

- 📊 **System Tray Widget** - Shows Razer Control status in the system tray
- ⚙️ **Quick Settings Access** - Left-click to open Razer Settings app
- 🎮 **Quick Control Menu** - Right-click for:
  - Fan Control
  - Power Profiles
  - RGB Control
  - Battery Health
  - Donation link ❤️

## Installation

### Requirements

- **KDE Plasma 6.0+** (tested on 6.2+)
- **Qt 6**
- **KDE Frameworks 6 (KF6)**
- Razer Control daemon and GUI installed (`razer-settings` in PATH)

### Easy Install (Recommended)

No compilation required! Just run:

```bash
cd razer_control_gui/kde-widget
./install-plasmoid.sh
```

This copies the QML files to `~/.local/share/plasma/plasmoids/` and rebuilds the KDE cache.

### Manual Install

```bash
# Copy plasmoid to user directory
mkdir -p ~/.local/share/plasma/plasmoids/com.github.encomjp.razercontrol
cp -r package/* ~/.local/share/plasma/plasmoids/com.github.encomjp.razercontrol/

# Rebuild KDE cache
kbuildsycoca6

# Optional: Restart Plasma shell
plasmashell --replace &
```

## Usage

### Adding to Panel

1. Right-click on your Plasma panel
2. Select "Add Widgets..." (or "Edit Panel" > "Add Widgets")
3. Search for "Razer Control"
4. Drag it to your panel

### Using the Widget

- **Left-click**: Opens the full Razer Settings application
- **Right-click**: Shows quick menu with options:
  - Fan Control
  - Power Profiles
  - RGB Control
  - Battery Health
  - Support Development ❤️ (donation link)

## Uninstall

```bash
rm -rf ~/.local/share/plasma/plasmoids/com.github.encomjp.razercontrol
kbuildsycoca6
```

## Directory Structure

```
kde-widget/
├── install-plasmoid.sh     # Easy installer (recommended)
├── README.md               # This file
└── package/
    ├── metadata.json       # Plasma 6 plugin metadata
    └── contents/
        └── ui/
            └── main.qml    # Widget QML interface
```

## Troubleshooting

### Widget doesn't appear in widget list

1. Verify installation: `ls ~/.local/share/plasma/plasmoids/ | grep razer`
2. Rebuild KDE cache: `kbuildsycoca6`
3. Restart Plasma: `plasmashell --replace &`

### Widget shows "Unsupported Widget"

This usually means the metadata.json is incompatible with your Plasma version. Ensure you're running **Plasma 6.0+**.

### "razer-settings" command not found

The Razer Control GUI must be installed and in your PATH. Install it first:
```bash
cd razer_control_gui
./install.sh install
```

## Development

The widget is pure QML - no compilation required. Edit `package/contents/ui/main.qml` and reinstall to test changes.

### Key Files

- `package/metadata.json` - Plugin metadata (Plasma 6 format)
- `package/contents/ui/main.qml` - Widget UI and logic

## License

GPLv2+ - Same as Razer Control project

## See Also

- [Razer Control Main Repository](https://github.com/encomjp/razercontrol-revived)
- [KDE Plasma Widget Development](https://develop.kde.org/plasma/)
