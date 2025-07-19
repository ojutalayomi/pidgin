#!/bin/bash

# Pidgin Compiler Installation Script
# This script installs the Pidgin compiler system-wide

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default installation directory
INSTALL_DIR="/usr/local/bin"
EXECUTABLE_NAME="pidgin-compiler"

echo -e "${BLUE}Pidgin Compiler Installation${NC}"
echo "================================"

# Check if we have a release build
if [ ! -f "target/release/pidgin-compiler" ]; then
    echo -e "${YELLOW}No release build found. Building now...${NC}"
    cargo build --release
fi

# Check if running as root for system-wide installation
if [ "$EUID" -eq 0 ]; then
    echo -e "${GREEN}Installing Pidgin compiler system-wide...${NC}"
    
    # Copy the executable
    cp target/release/pidgin-compiler "$INSTALL_DIR/$EXECUTABLE_NAME"
    chmod +x "$INSTALL_DIR/$EXECUTABLE_NAME"
    
    echo -e "${GREEN}✓ Pidgin compiler installed to $INSTALL_DIR/$EXECUTABLE_NAME${NC}"
    echo ""
    echo "You can now run:"
    echo "  $EXECUTABLE_NAME <file.pg>"
    echo ""
    echo "Examples:"
    echo "  $EXECUTABLE_NAME examples/hello.pg"
    echo "  $EXECUTABLE_NAME  # Start interactive REPL"
    
else
    echo -e "${YELLOW}For system-wide installation, run: sudo $0${NC}"
    echo ""
    echo -e "${BLUE}Alternative: Local installation${NC}"
    echo "================================"
    
    # Suggest local installation
    LOCAL_DIR="$HOME/.local/bin"
    
    echo "Would you like to install locally to $LOCAL_DIR? (y/n)"
    read -r response
    
    if [[ "$response" =~ ^[Yy]$ ]]; then
        mkdir -p "$LOCAL_DIR"
        cp target/release/pidgin-compiler "$LOCAL_DIR/$EXECUTABLE_NAME"
        chmod +x "$LOCAL_DIR/$EXECUTABLE_NAME"
        
        echo -e "${GREEN}✓ Pidgin compiler installed to $LOCAL_DIR/$EXECUTABLE_NAME${NC}"
        echo ""
        
        # Check if the directory is in PATH
        if [[ ":$PATH:" != *":$LOCAL_DIR:"* ]]; then
            echo -e "${YELLOW}Note: $LOCAL_DIR is not in your PATH.${NC}"
            echo "Add this line to your shell configuration file (~/.bashrc, ~/.zshrc, etc.):"
            echo -e "${BLUE}export PATH=\"$LOCAL_DIR:\$PATH\"${NC}"
            echo ""
            echo "Then restart your terminal or run:"
            echo -e "${BLUE}source ~/.bashrc${NC}"
        fi
        
        echo "You can now run:"
        echo "  $EXECUTABLE_NAME <file.pg>"
        
    else
        echo -e "${BLUE}Manual installation:${NC}"
        echo "========================"
        echo "1. Copy the executable to a directory in your PATH:"
        echo "   cp target/release/pidgin-compiler /path/to/directory/"
        echo ""
        echo "2. Make it executable:"
        echo "   chmod +x /path/to/directory/pidgin-compiler"
        echo ""
        echo "3. Or add the current directory to your PATH by adding this line to your shell config:"
        echo -e "${BLUE}export PATH=\"$PWD:\$PATH\"${NC}"
    fi
fi

echo ""
echo -e "${GREEN}Installation complete!${NC}" 