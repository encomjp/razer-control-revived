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

if [[ $EUID -ne 0 ]]; then
    print_error "This script must be run as root (use sudo)"
    exit 1
fi

echo "╔════════════════════════════════════════════════╗"
echo "║   Razer Control - Binary Installation          ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

# Install binaries
print_info "Installing binaries..."
install -D -m 755 "$SCRIPT_DIR/bin/razer-settings" /usr/local/bin/razer-settings
install -D -m 755 "$SCRIPT_DIR/bin/razer-cli" /usr/local/bin/razer-cli
install -D -m 755 "$SCRIPT_DIR/bin/daemon" /usr/local/bin/razer-daemon

# Install data files
print_info "Installing data files..."
install -D -m 644 "$SCRIPT_DIR/share/applications/razer-settings.desktop" /usr/share/applications/razer-settings.desktop
install -D -m 644 "$SCRIPT_DIR/share/razercontrol/laptops.json" /usr/share/razercontrol/laptops.json

# Install systemd service
print_info "Installing systemd service..."
install -D -m 644 "$SCRIPT_DIR/systemd/razercontrol.service" /etc/systemd/system/razercontrol.service

# Install udev rules
print_info "Installing udev rules..."
install -D -m 644 "$SCRIPT_DIR/udev/99-hidraw-permissions.rules" /etc/udev/rules.d/99-hidraw-permissions.rules

# Reload udev
print_info "Reloading udev rules..."
udevadm control --reload-rules
udevadm trigger

# Enable and start service
print_info "Enabling systemd service..."
systemctl daemon-reload
systemctl enable razercontrol
systemctl start razercontrol || print_warn "Could not start service - check 'systemctl status razercontrol'"

echo ""
print_info "Installation complete!"
echo ""
echo "To install the KDE Plasma widget (optional):"
echo "  1. Copy kde-widget/ to ~/.local/share/plasma/plasmoids/com.github.encomjp.razercontrol"
echo "  2. Run: kbuildsycoca6"
echo "  3. Add widget to your panel"
echo ""
echo "Please log out and back in (or reboot) for udev rules to take effect."
