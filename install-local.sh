#!/bin/bash
# Local installation script for razercontrol-revived

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/razer_control_gui"

if [ "$EUID" -eq 0 ]; then
    echo "Please do not run as root (sudo will be used where needed)"
    exit 1
fi

echo "Installing razercontrol-revived..."

# Install binaries (consistent with deb/rpm package names)
echo "Installing binaries to /usr/bin..."
sudo install -Dm755 "$BUILD_DIR/target/release/razer-settings" /usr/bin/razer-settings
sudo install -Dm755 "$BUILD_DIR/target/release/daemon" /usr/bin/razer-daemon
sudo install -Dm755 "$BUILD_DIR/target/release/razer-cli" /usr/bin/razer-cli

# Install desktop file
echo "Installing desktop entry..."
sudo install -Dm644 "$BUILD_DIR/data/gui/razer-settings.desktop" /usr/share/applications/razer-settings.desktop

# Install icon
if [ -f "$BUILD_DIR/data/gui/com.github.encomjp.razercontrol.svg" ]; then
    sudo install -Dm644 "$BUILD_DIR/data/gui/com.github.encomjp.razercontrol.svg" /usr/share/icons/hicolor/scalable/apps/com.github.encomjp.razercontrol.svg
fi

# Install udev rules
echo "Installing udev rules..."
sudo install -Dm644 "$BUILD_DIR/data/udev/99-hidraw-permissions.rules" /etc/udev/rules.d/99-hidraw-permissions.rules

# Install systemd user service
echo "Installing systemd user service..."
sudo install -Dm644 "$BUILD_DIR/data/services/systemd/razercontrol.service" /usr/lib/systemd/user/razercontrol.service

# Install device configuration
echo "Installing device configuration..."
sudo mkdir -p /usr/share/razercontrol
sudo install -Dm644 "$BUILD_DIR/data/devices/laptops.json" /usr/share/razercontrol/laptops.json

# Create config directory
mkdir -p ~/.local/share/razercontrol

# Reload udev and systemd
echo "Reloading udev rules..."
sudo udevadm control --reload-rules
sudo udevadm trigger

echo "Reloading systemd user daemon..."
systemctl --user daemon-reload

# Enable and start the user service
echo "Enabling and starting razercontrol daemon..."
systemctl --user enable razercontrol.service
systemctl --user restart razercontrol.service

echo ""
echo "Installation complete!"
echo ""
echo "You can now:"
echo "  - Search for 'Razer Settings' in your application menu"
echo "  - Run 'razer-settings' from the terminal"
echo "  - Run 'razer-cli' for CLI access"
echo ""
echo "The daemon is running as a systemd user service."
echo "To check status: systemctl --user status razercontrol"
