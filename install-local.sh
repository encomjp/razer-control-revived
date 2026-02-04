#!/bin/bash
# Local installation script for razercontrol-revived

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/razer_control_gui"

echo "Installing razercontrol-revived..."

# Install binaries
echo "Installing binaries to /usr/local/bin..."
sudo install -Dm755 "$BUILD_DIR/target/release/razer-settings" /usr/local/bin/razer-settings
sudo install -Dm755 "$BUILD_DIR/target/release/daemon" /usr/local/bin/razercontrol-daemon
sudo install -Dm755 "$BUILD_DIR/target/release/razer-cli" /usr/local/bin/razer-cli

# Install desktop file
echo "Installing desktop entry..."
sudo install -Dm644 "$BUILD_DIR/data/gui/razer-settings.desktop" /usr/share/applications/razer-settings.desktop

# Install udev rules
echo "Installing udev rules..."
sudo install -Dm644 "$BUILD_DIR/data/udev/99-hidraw-permissions.rules" /etc/udev/rules.d/99-hidraw-permissions.rules

# Install systemd service
echo "Installing systemd service..."
sudo install -Dm644 "$BUILD_DIR/data/services/systemd/razercontrol.service" /etc/systemd/system/razercontrol.service

# Install device configuration
echo "Installing device configuration..."
sudo mkdir -p /usr/share/razercontrol
sudo install -Dm644 "$BUILD_DIR/data/devices/laptops.json" /usr/share/razercontrol/laptops.json

# Reload udev and systemd
echo "Reloading udev rules..."
sudo udevadm control --reload-rules
sudo udevadm trigger

echo "Reloading systemd..."
sudo systemctl daemon-reload

# Enable and start the daemon
echo "Enabling and starting razercontrol daemon..."
sudo systemctl enable razercontrol.service
sudo systemctl restart razercontrol.service

echo ""
echo "Installation complete!"
echo ""
echo "You can now:"
echo "  - Search for 'Razer Settings' in your application menu"
echo "  - Run 'razer-settings' from the terminal"
echo "  - Run 'razer-cli' for CLI access"
echo ""
echo "The daemon is running as a systemd service."
