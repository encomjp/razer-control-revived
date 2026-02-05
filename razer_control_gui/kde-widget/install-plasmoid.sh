#!/bin/bash

# Razer Control KDE Plasmoid Installation Script
# This script installs a QML-only Plasma widget (no compilation required)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGE_DIR="$SCRIPT_DIR/package"
PLASMOID_ID="com.github.encomjp.razercontrol"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if the package directory exists
if [ ! -d "$PACKAGE_DIR" ]; then
    print_error "Package directory not found: $PACKAGE_DIR"
    exit 1
fi

# Check for required files
if [ ! -f "$PACKAGE_DIR/metadata.json" ]; then
    print_error "metadata.json not found in package directory"
    exit 1
fi

if [ ! -f "$PACKAGE_DIR/contents/ui/main.qml" ]; then
    print_error "main.qml not found in package directory"
    exit 1
fi

echo "╔════════════════════════════════════════════════╗"
echo "║   Razer Control KDE Plasmoid Installation      ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

# Remove old installation if it exists
INSTALL_DIR="$HOME/.local/share/plasma/plasmoids/$PLASMOID_ID"
if [ -d "$INSTALL_DIR" ]; then
    print_info "Removing existing installation..."
    rm -rf "$INSTALL_DIR"
fi

# Create installation directory
print_info "Installing plasmoid to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

# Copy package contents
cp -r "$PACKAGE_DIR"/* "$INSTALL_DIR/"

print_info "Plasmoid installed successfully!"
echo ""

# Rebuild KDE cache
print_info "Rebuilding KDE cache..."
if command -v kbuildsycoca6 &> /dev/null; then
    kbuildsycoca6 --noincremental > /dev/null 2>&1 || true
elif command -v kbuildsycoca5 &> /dev/null; then
    kbuildsycoca5 --noincremental > /dev/null 2>&1 || true
fi

# Setup autostart (optional)
read -p "Do you want to enable auto-start for razer-settings? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    AUTOSTART_DIR="$HOME/.config/autostart"
    mkdir -p "$AUTOSTART_DIR"
    
    cat > "$AUTOSTART_DIR/razer-settings.desktop" << 'EOF'
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
EOF
    
    print_info "Auto-start entry created."
fi

echo ""
echo -e "${GREEN}Installation Complete!${NC}"
echo ""
echo "To use the widget:"
echo "1. Right-click on your Plasma panel"
echo "2. Select 'Enter Edit Mode' or 'Edit Panel...'"
echo "3. Click '+' or 'Add Widgets'"
echo "4. Search for 'Razer Control'"
echo "5. Drag it to your panel or desktop"
echo ""
echo "The widget will appear with a power icon."
echo "Left-click to open settings, right-click for quick menu."
echo ""

# Optionally restart plasmashell
read -p "Do you want to restart Plasma Shell now to load the widget? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "Restarting Plasma Shell..."
    if command -v kquitapp6 &> /dev/null; then
        kquitapp6 plasmashell > /dev/null 2>&1 || true
        sleep 2
        kstart6 plasmashell > /dev/null 2>&1 &
    elif command -v kquitapp5 &> /dev/null; then
        kquitapp5 plasmashell > /dev/null 2>&1 || true
        sleep 2
        kstart5 plasmashell > /dev/null 2>&1 &
    else
        print_warn "Could not restart Plasma Shell automatically."
        print_warn "Please log out and log back in to see the widget."
    fi
    print_info "Plasma Shell restarted."
fi
