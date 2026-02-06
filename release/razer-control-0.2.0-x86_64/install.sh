#!/bin/bash
# Razer Control - Pre-compiled Binary Installation Script
# This installs the pre-compiled binaries to your system

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
print_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ $EUID -eq 0 ]]; then
    print_error "Please do not run as root (sudo will be used where needed)"
    exit 1
fi

echo "╔════════════════════════════════════════════════╗"
echo "║   Razer Control - Binary Installation          ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

# Install binaries
print_info "Installing binaries..."
sudo install -D -m 755 "$SCRIPT_DIR/bin/razer-settings" /usr/bin/razer-settings
sudo install -D -m 755 "$SCRIPT_DIR/bin/razer-cli" /usr/bin/razer-cli
sudo install -D -m 755 "$SCRIPT_DIR/bin/daemon" /usr/bin/razer-daemon

# Install data files
print_info "Installing data files..."
sudo install -D -m 644 "$SCRIPT_DIR/share/applications/razer-settings.desktop" /usr/share/applications/razer-settings.desktop
sudo install -D -m 644 "$SCRIPT_DIR/share/razercontrol/laptops.json" /usr/share/razercontrol/laptops.json

# Install systemd user service
print_info "Installing systemd user service..."
sudo install -D -m 644 "$SCRIPT_DIR/systemd/razercontrol.service" /usr/lib/systemd/user/razercontrol.service

# Install udev rules
print_info "Installing udev rules..."
sudo install -D -m 644 "$SCRIPT_DIR/udev/99-hidraw-permissions.rules" /etc/udev/rules.d/99-hidraw-permissions.rules

# Create config directory
mkdir -p ~/.local/share/razercontrol

# Reload udev
print_info "Reloading udev rules..."
sudo udevadm control --reload-rules
sudo udevadm trigger

# Enable and start user service
print_info "Enabling systemd user service..."
systemctl --user daemon-reload
systemctl --user enable razercontrol
systemctl --user start razercontrol || print_warn "Could not start service - check 'systemctl --user status razercontrol'"

echo ""
print_info "Installation complete!"
echo ""
echo "To install the KDE Plasma widget (optional):"
echo "  1. Copy kde-widget/ to ~/.local/share/plasma/plasmoids/com.github.encomjp.razercontrol"
echo "  2. Run: kbuildsycoca6"
echo "  3. Add widget to your panel"
echo ""
echo "Please log out and back in (or reboot) for udev rules to take effect."
echo "To check daemon status: systemctl --user status razercontrol"
