#!/bin/bash

# Pidgin Compiler Distribution Script
# This script creates a portable distribution of the Pidgin compiler

set -e

echo "Creating Pidgin Compiler distribution..."

# Detect platform
PLATFORM=""
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [[ $(uname -m) == "x86_64" ]]; then
        PLATFORM="linux-x86_64"
    elif [[ $(uname -m) == "aarch64" ]]; then
        PLATFORM="linux-aarch64"
    else
        echo "Unsupported Linux architecture"
        exit 1
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if [[ $(uname -m) == "x86_64" ]]; then
        PLATFORM="macos-x86_64"
    elif [[ $(uname -m) == "arm64" ]]; then
        PLATFORM="macos-aarch64"
    else
        echo "Unsupported macOS architecture"
        exit 1
    fi
else
    echo "Unsupported operating system: $OSTYPE"
    exit 1
fi

echo "Detected platform: $PLATFORM"

# Use the new distribution script
if [ -f "scripts/create-distribution.sh" ]; then
    chmod +x scripts/create-distribution.sh
    ./scripts/create-distribution.sh "$PLATFORM" "$OSTYPE"
    
    # Create zip archive
    echo "Creating distribution archive..."
    zip -r "pidgin-$PLATFORM.zip" "pidgin-$PLATFORM"
    
    echo "Distribution created successfully!"
    echo "Files created:"
    echo "  - pidgin-$PLATFORM/ (distribution directory)"
    echo "  - pidgin-$PLATFORM.zip (archive)"
    echo ""
    echo "To distribute:"
    echo "  1. Copy the zip file to any computer"
    echo "  2. Extract it"
    echo "  3. Run: ./run.sh examples/hello.pg"
else
    echo "Error: scripts/create-distribution.sh not found"
    exit 1
fi 