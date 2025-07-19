#!/bin/bash

# Pidgin Compiler Distribution Script
# This script creates a portable distribution of the Pidgin compiler

set -e

echo "Creating Pidgin Compiler distribution..."

# Create distribution directory
DIST_DIR="pidgin-compiler-dist"
mkdir -p "$DIST_DIR"

# Copy the release executable
cp target/release/pidgin-compiler "$DIST_DIR/"

# Copy example files
mkdir -p "$DIST_DIR/examples"
cp examples/*.pg "$DIST_DIR/examples/"

# Create a simple runner script for Unix-like systems
cat > "$DIST_DIR/run.sh" << 'EOF'
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
EXECUTABLE="$SCRIPT_DIR/pidgin-compiler"

# Check if the executable exists
if [ ! -f "$EXECUTABLE" ]; then
    echo "Error: pidgin-compiler executable not found in $SCRIPT_DIR"
    exit 1
fi

# Make sure the executable is executable
chmod +x "$EXECUTABLE"

# Run the compiler
"$EXECUTABLE" "$@"
EOF

# Make the runner script executable
chmod +x "$DIST_DIR/run.sh"

# Create a Windows batch file
cat > "$DIST_DIR/run.bat" << 'EOF'
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
set EXECUTABLE=%SCRIPT_DIR%pidgin-compiler.exe

REM Check if the executable exists
if not exist "%EXECUTABLE%" (
    echo Error: pidgin-compiler.exe not found in %SCRIPT_DIR%
    exit /b 1
)

REM Run the compiler
"%EXECUTABLE%" %*
EOF

# Create a README for the distribution
cat > "$DIST_DIR/README.md" << 'EOF'
# Pidgin Compiler - Portable Distribution

This is a portable distribution of the Pidgin programming language compiler.

## Quick Start

### On Unix-like systems (Linux, macOS):
```bash
./run.sh examples/hello.pg
```

### On Windows:
```cmd
run.bat examples\hello.pg
```

### Direct execution:
```bash
./pidgin-compiler examples/hello.pg
```

## What's Included

- `pidgin-compiler` - The main executable
- `run.sh` - Unix/Linux/macOS runner script
- `run.bat` - Windows runner script
- `examples/` - Example Pidgin programs
- `README.md` - This file

## Examples

Try running some of the included examples:

```bash
# Hello World
./run.sh examples/hello.pg

# Fibonacci sequence
./run.sh examples/fibonacci.pg

# Simple arithmetic
./run.sh examples/simple.pg
```

## Interactive Mode

To start the interactive REPL:

```bash
./run.sh
```

## Language Features

- Variables: `let x = 10;`
- Arithmetic: `+`, `-`, `*`, `/`
- Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Conditionals: `if`, `else`
- Loops: `while`
- String concatenation: `"Hello " + "World"`
- Comments: `// This is a comment`
- Print statements: `print "Hello, World!";`

## Troubleshooting

1. **Permission denied**: Make sure the executable has execute permissions:
   ```bash
   chmod +x pidgin-compiler
   ```

2. **File not found**: Make sure you're in the correct directory and the file path is correct.

3. **Syntax errors**: Check the error messages for line and column information to locate issues in your code.

## Building from Source

If you want to build the compiler from source:

1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Run: `cargo build --release`
4. The executable will be in `target/release/pidgin-compiler`
EOF

# Create a simple installation script
cat > "$DIST_DIR/install.sh" << 'EOF'
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
EOF

chmod +x "$DIST_DIR/install.sh"

# Create a zip archive
echo "Creating distribution archive..."
zip -r "pidgin-compiler-$(uname -s)-$(uname -m).zip" "$DIST_DIR"

echo "Distribution created successfully!"
echo "Files created:"
echo "  - $DIST_DIR/ (distribution directory)"
echo "  - pidgin-compiler-$(uname -s)-$(uname -m).zip (archive)"
echo ""
echo "To distribute:"
echo "  1. Copy the zip file to any computer"
echo "  2. Extract it"
echo "  3. Run: ./run.sh examples/hello.pg" 