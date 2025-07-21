#!/bin/bash

# Pidgin Compiler Update Script
# This script helps you update the Pidgin compiler to the latest version

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Pidgin Compiler Update Script${NC}"
echo "================================"

# Detect platform
PLATFORM=""
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [[ $(uname -m) == "x86_64" ]]; then
        PLATFORM="linux-x86_64"
    elif [[ $(uname -m) == "aarch64" ]]; then
        PLATFORM="linux-aarch64"
    else
        echo -e "${RED}Unsupported Linux architecture${NC}"
        exit 1
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if [[ $(uname -m) == "x86_64" ]]; then
        PLATFORM="macos-x86_64"
    elif [[ $(uname -m) == "arm64" ]]; then
        PLATFORM="macos-aarch64"
    else
        echo -e "${RED}Unsupported macOS architecture${NC}"
        exit 1
    fi
else
    echo -e "${RED}Unsupported operating system: $OSTYPE${NC}"
    exit 1
fi

echo -e "${GREEN}Detected platform: $PLATFORM${NC}"

# Find current installation
CURRENT_PATH=""
if command -v pidgin >/dev/null 2>&1; then
    CURRENT_PATH=$(which pidgin)
    echo -e "${GREEN}Current installation found at: $CURRENT_PATH${NC}"
else
    echo -e "${YELLOW}No current installation found${NC}"
fi

# Get latest version
echo "Checking for latest version..."
LATEST_VERSION=$(curl -s https://api.github.com/repos/ojutalayomi/pidgin/releases/latest | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$LATEST_VERSION" ]; then
    echo -e "${RED}Failed to get latest version${NC}"
    exit 1
fi

echo -e "${GREEN}Latest version: $LATEST_VERSION${NC}"

# Check if we need to update
if [ -n "$CURRENT_PATH" ]; then
    CURRENT_VERSION=$(pidgin --version 2>/dev/null | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "unknown")
    echo -e "${GREEN}Current version: $CURRENT_VERSION${NC}"
    
    if [ "$CURRENT_VERSION" = "$LATEST_VERSION" ]; then
        echo -e "${GREEN}You already have the latest version!${NC}"
        exit 0
    fi
fi

# Download latest release
DOWNLOAD_URL="https://github.com/ojutalayomi/pidgin/releases/download/$LATEST_VERSION/pidgin-$PLATFORM.zip"
TEMP_DIR=$(mktemp -d)

echo "Downloading latest release..."
if ! curl -L -o "$TEMP_DIR/pidgin-$PLATFORM.zip" "$DOWNLOAD_URL"; then
    echo -e "${RED}Failed to download latest release${NC}"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Extract the release
echo "Extracting release..."
cd "$TEMP_DIR"
unzip -q "pidgin-$PLATFORM.zip"
cd "pidgin-$PLATFORM"

# Install the update
echo "Installing update..."
if [ -f "install.sh" ]; then
    chmod +x install.sh
    if [ -w "$(dirname "$CURRENT_PATH")" ]; then
        # We can write to the directory, so do a direct update
        echo "Updating existing installation..."
        cp pidgin "$CURRENT_PATH"
        chmod +x "$CURRENT_PATH"
        echo -e "${GREEN}✓ Update completed successfully!${NC}"
    else
        # Need sudo for system-wide installation
        echo "Updating system-wide installation..."
        sudo ./install.sh
    fi
else
    echo -e "${RED}Installation script not found in release${NC}"
    exit 1
fi

# Clean up
cd /
rm -rf "$TEMP_DIR"

# Verify the update
if command -v pidgin >/dev/null 2>&1; then
    NEW_VERSION=$(pidgin --version 2>/dev/null | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "unknown")
    echo -e "${GREEN}✓ Update verified! New version: $NEW_VERSION${NC}"
else
    echo -e "${YELLOW}Warning: Could not verify the update${NC}"
fi

echo ""
echo -e "${BLUE}Update complete!${NC}"
echo "You can now use: pidgin --version" 