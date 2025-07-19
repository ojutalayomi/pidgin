#!/bin/bash

# Pidgin Compiler Installation Script
# This script installs the Pidgin compiler to a system-wide location

set -e

# Default installation directory
INSTALL_DIR="/usr/local/bin"

# Check if running as root for system-wide installation
if [ "$EUID" -eq 0 ]; then
    echo "Installing Pidgin compiler system-wide..."
    cp pidgin-compiler "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/pidgin-compiler"
    echo "Pidgin compiler installed to $INSTALL_DIR/pidgin-compiler"
    echo "You can now run: pidgin-compiler <file.pg>"
else
    echo "For system-wide installation, run: sudo $0"
    echo "Or install locally by adding the current directory to your PATH"
    echo "Add this line to your ~/.bashrc or ~/.zshrc:"
    echo "export PATH=\"$PWD:\$PATH\""
fi
