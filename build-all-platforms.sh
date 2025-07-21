#!/bin/bash

# Cross-platform build script for Pidgin Compiler
# This script builds the compiler for multiple platforms

set -e

echo "Building Pidgin Compiler for multiple platforms..."

# Check if rustup is installed
if ! command -v rustup &> /dev/null; then
    echo "Error: rustup is not installed. Please install it from https://rustup.rs/"
    exit 1
fi

# Install required targets
echo "Installing required targets..."
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-apple-darwin
rustup target add aarch64-unknown-linux-gnu

# Create builds directory
BUILDS_DIR="builds"
mkdir -p "$BUILDS_DIR"

# Function to build for a specific target
build_for_target() {
    local target=$1
    local platform_name=$2
    
    echo "Building for $platform_name ($target)..."
    
    # Build the release version
    cargo build --release --target "$target"
    
    # Create platform-specific directory
    local platform_dir="$BUILDS_DIR/$platform_name"
    mkdir -p "$platform_dir"
    
    # Copy the executable
    if [[ "$target" == *"windows"* ]]; then
        cp "target/$target/release/pidgin.exe" "$platform_dir/"
    else
        cp "target/$target/release/pidgin" "$platform_dir/"
    fi
    
    # Copy examples
    mkdir -p "$platform_dir/examples"
    cp examples/*.pg "$platform_dir/examples/"
    
    # Create platform-specific runner scripts
    if [[ "$target" == *"windows"* ]]; then
        # Windows batch file
        cat > "$platform_dir/run.bat" << 'EOF'
@echo off
REM Pidgin Compiler Runner Script for Windows
REM Usage: run.bat <file.pg>

if "%~1"=="" (
    echo Usage: %0 ^<file.pg^>
    echo Example: %0 examples\hello.pg
    exit /b 1
)

REM Get the directory where this script is located
set SCRIPT_DIR=%~dp0
set EXECUTABLE=%SCRIPT_DIR%pidgin.exe

REM Check if the executable exists
if not exist "%EXECUTABLE%" (
    echo Error: pidgin.exe not found in %SCRIPT_DIR%
    exit /b 1
)

REM Run the compiler
"%EXECUTABLE%" %*
EOF
    else
        # Unix shell script
        cat > "$platform_dir/run.sh" << 'EOF'
#!/bin/bash

# Pidgin Compiler Runner Script
# Usage: ./run.sh <file.pg>

if [ $# -eq 0 ]; then
    echo "Usage: $0 <file.pg>"
    echo "Example: $0 examples/hello.pg"
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXECUTABLE="$SCRIPT_DIR/pidgin"

# Check if the executable exists
if [ ! -f "$EXECUTABLE" ]; then
    echo "Error: pidgin executable not found in $SCRIPT_DIR"
    exit 1
fi

# Make sure the executable is executable
chmod +x "$EXECUTABLE"

# Run the compiler
"$EXECUTABLE" "$@"
EOF
        chmod +x "$platform_dir/run.sh"
    fi
    
    # Create README for this platform
    cat > "$platform_dir/README.md" << EOF
# Pidgin Compiler - $platform_name

This is the Pidgin compiler built for $platform_name.

## Quick Start

EOF
    
    if [[ "$target" == *"windows"* ]]; then
        cat >> "$platform_dir/README.md" << 'EOF'
### On Windows:
```cmd
run.bat examples\hello.pg
```

### Direct execution:
```cmd
pidgin.exe examples\hello.pg
```
EOF
    else
        cat >> "$platform_dir/README.md" << 'EOF'
### On Unix-like systems (Linux, macOS):
```bash
./run.sh examples/hello.pg
```

### Direct execution:
```bash
./pidgin examples/hello.pg
```
EOF
    fi
    
    cat >> "$platform_dir/README.md" << 'EOF'

## Examples

Try running some of the included examples:

EOF
    
    if [[ "$target" == *"windows"* ]]; then
        cat >> "$platform_dir/README.md" << 'EOF'
```cmd
# Hello World
run.bat examples\hello.pg

# Fibonacci sequence
run.bat examples\fibonacci.pg

# Simple arithmetic
run.bat examples\simple.pg
```
EOF
    else
        cat >> "$platform_dir/README.md" << 'EOF'
```bash
# Hello World
./run.sh examples/hello.pg

# Fibonacci sequence
./run.sh examples/fibonacci.pg

# Simple arithmetic
./run.sh examples/simple.pg
```
EOF
    fi
    
    cat >> "$platform_dir/README.md" << 'EOF'

## Interactive Mode

To start the interactive REPL:

EOF
    
    if [[ "$target" == *"windows"* ]]; then
        cat >> "$platform_dir/README.md" << 'EOF'
```cmd
run.bat
```
EOF
    else
        cat >> "$platform_dir/README.md" << 'EOF'
```bash
./run.sh
```
EOF
    fi
    
    cat >> "$platform_dir/README.md" << 'EOF'

## Language Features

- Variables: `let x = 10;`
- Arithmetic: `+`, `-`, `*`, `/`
- Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Conditionals: `if`, `else`
- Loops: `while`
- String concatenation: `"Hello " + "World"`
- Comments: `// This is a comment`
- Print statements: `print "Hello, World!";`
EOF
    
    echo "✓ Built for $platform_name"
}

# Build for different platforms
build_for_target "x86_64-unknown-linux-gnu" "linux-x86_64"
build_for_target "x86_64-apple-darwin" "macos-x86_64"
build_for_target "x86_64-pc-windows-gnu" "windows-x86_64"
build_for_target "aarch64-apple-darwin" "macos-aarch64"
build_for_target "aarch64-unknown-linux-gnu" "linux-aarch64"

# Create a master README
cat > "$BUILDS_DIR/README.md" << 'EOF'
# Pidgin Compiler - Multi-Platform Builds

This directory contains builds of the Pidgin compiler for different platforms and architectures.

## Available Builds

- `linux-x86_64/` - Linux on x86_64 (Intel/AMD 64-bit)
- `linux-aarch64/` - Linux on ARM64 (ARM 64-bit)
- `macos-x86_64/` - macOS on x86_64 (Intel Macs)
- `macos-aarch64/` - macOS on ARM64 (Apple Silicon Macs)
- `windows-x86_64/` - Windows on x86_64 (Intel/AMD 64-bit)

## Usage

1. Choose the appropriate build for your system
2. Extract or copy the files to your desired location
3. Follow the instructions in the platform-specific README.md

## Quick Start

### Linux (x86_64):
```bash
cd linux-x86_64
./run.sh examples/hello.pg
```

### macOS (Intel):
```bash
cd macos-x86_64
./run.sh examples/hello.pg
```

### macOS (Apple Silicon):
```bash
cd macos-aarch64
./run.sh examples/hello.pg
```

### Windows:
```cmd
cd windows-x86_64
run.bat examples\hello.pg
```

## Building from Source

If you want to build for a specific platform:

1. Install Rust: https://rustup.rs/
2. Install the target: `rustup target add <target>`
3. Build: `cargo build --release --target <target>`

Common targets:
- `x86_64-unknown-linux-gnu` - Linux x86_64
- `x86_64-apple-darwin` - macOS Intel
- `x86_64-pc-windows-gnu` - Windows x86_64
- `aarch64-apple-darwin` - macOS Apple Silicon
- `aarch64-unknown-linux-gnu` - Linux ARM64
EOF

# Create distribution archives
echo "Creating distribution archives..."
cd "$BUILDS_DIR"

for platform in */; do
    platform=${platform%/}
    echo "Creating archive for $platform..."
    zip -r "../pidgin-$platform.zip" "$platform"
done

cd ..

echo ""
echo "Build completed successfully!"
echo "Distribution archives created:"
for archive in pidgin-*.zip; do
    if [ -f "$archive" ]; then
        echo "  - $archive"
    fi
done
echo ""
echo "To distribute:"
echo "  1. Copy the appropriate zip file for your target platform"
echo "  2. Extract it on the target computer"
echo "  3. Follow the instructions in the README.md file" 