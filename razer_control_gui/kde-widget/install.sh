#!/bin/bash

# Razer Control KDE Widget Installation Script
# This script builds and installs the KDE Plasma widget for Razer Control

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"
INSTALL_PREFIX="${KDE_INSTALL_PREFIX:-/usr/local}"

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

# Check if required commands are available
check_requirements() {
    local missing_tools=()
    
    for tool in cmake make g++ pkg-config; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done
    
    if [ ${#missing_tools[@]} -gt 0 ]; then
        print_error "Missing required tools: ${missing_tools[*]}"
        echo "Please install them using your package manager:"
        echo "  Ubuntu/Debian: sudo apt install build-essential cmake"
        echo "  Fedora/RHEL: sudo dnf install gcc-c++ cmake"
        echo "  Arch: sudo pacman -S base-devel cmake"
        exit 1
    fi
    
    # Check for Qt6 or Qt5
    if ! pkg-config --exists Qt6Core 2>/dev/null && ! pkg-config --exists Qt5Core 2>/dev/null; then
        print_error "Qt6 or Qt5 development libraries not found"
        echo "Please install Qt development files:"
        echo "  Ubuntu/Debian: sudo apt install qt6-base-dev (or qt5-qmake qtbase5-dev)"
        echo "  Fedora/RHEL: sudo dnf install qt6-qtbase-devel (or qt5-qtbase-devel)"
        exit 1
    fi
    
    # Check for KDE Frameworks
    if ! pkg-config --exists KF6CoreAddons 2>/dev/null && ! pkg-config --exists KF5CoreAddons 2>/dev/null; then
        print_error "KDE Frameworks development libraries not found"
        echo "Please install KDE development files:"
        echo "  Ubuntu/Debian: sudo apt install extra-cmake-modules libkf6plasma-dev libkf6i18n-dev qt6-base-dev"
        echo "  Fedora/RHEL: sudo dnf install extra-cmake-modules libplasma-devel kf6-ki18n-devel qt6-qtbase-devel"
        exit 1
    fi
}

# Create build directory
setup_build() {
    print_info "Setting up build directory..."
    
    if [ -d "$BUILD_DIR" ]; then
        print_warn "Build directory already exists, removing it..."
        rm -rf "$BUILD_DIR"
    fi
    
    mkdir -p "$BUILD_DIR"
    cd "$BUILD_DIR"
}

# Configure and build
build() {
    print_info "Configuring build with CMake..."
    cmake -DCMAKE_INSTALL_PREFIX="$INSTALL_PREFIX" ..
    
    print_info "Building KDE widget..."
    make -j$(nproc)
}

# Install
install() {
    print_info "Installing KDE widget..."
    
    # Try with sudo if not running as root
    if [ "$EUID" -ne 0 ]; then
        print_warn "Not running as root, will use sudo for installation"
        sudo make install
    else
        make install
    fi
    
    print_info "Rebuilding KDE system configuration cache..."
    if command -v kbuildsycoca6 &> /dev/null; then
        kbuildsycoca6 > /dev/null 2>&1 || true
    elif command -v kbuildsycoca5 &> /dev/null; then
        kbuildsycoca5 > /dev/null 2>&1 || true
    fi
}

# Setup autostart
setup_autostart() {
    print_info "Setting up auto-start configuration..."
    
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

    chmod +x "$AUTOSTART_DIR/razer-settings.desktop"
    print_info "Auto-start entry created at $AUTOSTART_DIR/razer-settings.desktop"
}

# Post-install steps
post_install() {
    print_info "Post-installation steps..."
    
    echo ""
    echo -e "${GREEN}Installation Complete!${NC}"
    echo ""
    echo "To use the widget:"
    echo "1. Right-click on your Plasma panel"
    echo "2. Select 'Edit Panel...'"
    echo "3. Click '+' to add a widget"
    echo "4. Search for 'Razer Control'"
    echo "5. Click to add it to your panel"
    echo ""
    echo "The widget will appear in your system tray (bottom-right corner)"
    echo "Left-click to open full settings, right-click for quick menu"
    echo ""
    
    # Ask about restarting Plasma
    read -p "Do you want to restart Plasma now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_info "Restarting Plasma..."
        if command -v kquitapp6 &> /dev/null; then
            kquitapp6 plasmashell > /dev/null 2>&1 || true
            sleep 2
            kstart6 plasmashell > /dev/null 2>&1 &
        elif command -v kquitapp5 &> /dev/null; then
            kquitapp5 plasmashell > /dev/null 2>&1 || true
            sleep 2
            kstart5 plasmashell > /dev/null 2>&1 &
        fi
        print_info "Plasma restarted successfully"
    fi
}

# Main execution
main() {
    echo "╔════════════════════════════════════════════════╗"
    echo "║   Razer Control KDE Widget Installation        ║"
    echo "╚════════════════════════════════════════════════╝"
    echo ""
    
    check_requirements
    setup_build
    build
    install
    setup_autostart
    post_install
}

main "$@"
