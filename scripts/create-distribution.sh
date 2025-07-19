#!/bin/bash

# Create distribution script for GitHub Actions
# Usage: ./create-distribution.sh <platform> <os>

set -e

PLATFORM=$1
OS=$2

if [ -z "$PLATFORM" ] || [ -z "$OS" ]; then
    echo "Usage: $0 <platform> <os>"
    echo "Example: $0 linux-x86_64 ubuntu-latest"
    exit 1
fi

echo "Creating distribution for $PLATFORM on $OS..."

# Create distribution directory
mkdir -p "pidgin-compiler-$PLATFORM"

# Copy executable
if [ "$OS" = "windows-latest" ]; then
    cp target/x86_64-pc-windows-msvc/release/pidgin-compiler.exe "pidgin-compiler-$PLATFORM/"
else
    cp target/*/release/pidgin-compiler "pidgin-compiler-$PLATFORM/"
fi

# Copy examples
cp -r examples "pidgin-compiler-$PLATFORM/"

# Create runner scripts
if [ "$OS" = "windows-latest" ]; then
    cat > "pidgin-compiler-$PLATFORM/run.bat" << 'EOF'
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
else
    cat > "pidgin-compiler-$PLATFORM/run.sh" << 'EOF'
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
    chmod +x "pidgin-compiler-$PLATFORM/run.sh"
fi

# Create README
cat > "pidgin-compiler-$PLATFORM/README.md" << EOF
# Pidgin Compiler - $PLATFORM

This is the Pidgin compiler built for $PLATFORM.

## Quick Start

EOF

if [ "$OS" = "windows-latest" ]; then
    cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'
### On Windows:
```cmd
run.bat examples\hello.pg
```

### Direct execution:
```cmd
pidgin-compiler.exe examples\hello.pg
```
EOF
else
    cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'
### On Unix-like systems (Linux, macOS):
```bash
./run.sh examples/hello.pg
```

### Direct execution:
```bash
./pidgin-compiler examples/hello.pg
```
EOF
fi

cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'

## Examples

Try running some of the included examples:

EOF

if [ "$OS" = "windows-latest" ]; then
    cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'
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
    cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'
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

cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'

## Interactive Mode

To start the interactive REPL:

EOF

if [ "$OS" = "windows-latest" ]; then
    cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'
```cmd
run.bat
```
EOF
else
    cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'
```bash
./run.sh
```
EOF
fi

cat >> "pidgin-compiler-$PLATFORM/README.md" << 'EOF'

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

echo "Distribution created successfully for $PLATFORM" 