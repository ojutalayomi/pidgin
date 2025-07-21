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

# Debug: Show current directory and target structure
echo "=== Debug Information ==="
echo "Current directory: $(pwd)"
echo "Target directory exists: $(test -d target && echo "YES" || echo "NO")"
if [ -d target ]; then
    echo "Target contents:"
    find target -name "pidgin*" -type f 2>/dev/null || echo "No pidgin files found"
    echo "Release directory exists: $(test -d target/release && echo "YES" || echo "NO")"
    if [ -d target/release ]; then
        echo "Release directory contents:"
        ls -la target/release/
    fi
fi
echo "========================"

# Create distribution directory
mkdir -p "pidgin-$PLATFORM"

# Copy executable
if [ "$OS" = "windows-latest" ]; then
    # For Windows, look for the specific target
    if [ -f "target/x86_64-pc-windows-msvc/release/pidgin.exe" ]; then
        echo "Found executable at target/x86_64-pc-windows-msvc/release/pidgin.exe"
        cp target/x86_64-pc-windows-msvc/release/pidgin.exe "pidgin-$PLATFORM/"
    else
        echo "Error: Windows executable not found at target/x86_64-pc-windows-msvc/release/pidgin.exe"
        exit 1
    fi
else
    # For Unix-like systems, try to find the executable
    if [ -f "target/release/pidgin" ]; then
        echo "Found executable at target/release/pidgin"
        cp target/release/pidgin "pidgin-$PLATFORM/"
    elif [ -f "target/x86_64-unknown-linux-gnu/release/pidgin" ]; then
        echo "Found executable at target/x86_64-unknown-linux-gnu/release/pidgin"
        cp target/x86_64-unknown-linux-gnu/release/pidgin "pidgin-$PLATFORM/"
    elif [ -f "target/aarch64-unknown-linux-gnu/release/pidgin" ]; then
        echo "Found executable at target/aarch64-unknown-linux-gnu/release/pidgin"
        cp target/aarch64-unknown-linux-gnu/release/pidgin "pidgin-$PLATFORM/"
    elif [ -f "target/x86_64-apple-darwin/release/pidgin" ]; then
        echo "Found executable at target/x86_64-apple-darwin/release/pidgin"
        cp target/x86_64-apple-darwin/release/pidgin "pidgin-$PLATFORM/"
    elif [ -f "target/aarch64-apple-darwin/release/pidgin" ]; then
        echo "Found executable at target/aarch64-apple-darwin/release/pidgin"
        cp target/aarch64-apple-darwin/release/pidgin "pidgin-$PLATFORM/"
    else
        echo "Error: Executable not found. Available targets:"
        find target -name "pidgin*" -type f 2>/dev/null || echo "No targets found"
        echo "Tried paths:"
        echo "  - target/release/pidgin"
        echo "  - target/x86_64-unknown-linux-gnu/release/pidgin"
        echo "  - target/aarch64-unknown-linux-gnu/release/pidgin"
        echo "  - target/x86_64-apple-darwin/release/pidgin"
        echo "  - target/aarch64-apple-darwin/release/pidgin"
        exit 1
    fi
fi

# Copy examples
cp -r examples "pidgin-$PLATFORM/"

# Copy documentation files
if [ -f "CHANGELOG.md" ]; then
    echo "Copying CHANGELOG.md"
    cp CHANGELOG.md "pidgin-$PLATFORM/"
fi

if [ -f "README.md" ]; then
    echo "Copying README.md"
    cp README.md "pidgin-$PLATFORM/"
fi

if [ -f "LICENSE" ]; then
    echo "Copying LICENSE"
    cp LICENSE "pidgin-$PLATFORM/"
fi

# Copy installation script (for Unix-like systems)
if [ "$OS" != "windows-latest" ]; then
    if [ -f "install.sh" ]; then
        echo "Copying install.sh"
        cp install.sh "pidgin-$PLATFORM/"
        chmod +x "pidgin-$PLATFORM/install.sh"
    else
        echo "Warning: install.sh not found, skipping"
    fi
    
    # Note: Update functionality is now integrated into pidgin command
    echo "Update functionality is integrated into pidgin command"
fi

# Create Windows installation script
if [ "$OS" = "windows-latest" ]; then
    echo "Creating Windows install.bat"
    cat > "pidgin-$PLATFORM/install.bat" << 'EOF'
@echo off
REM Pidgin Compiler Windows Installation Script
REM This script installs the pidgin to a system-wide location

echo Installing Pidgin Compiler for Windows...

REM Get the directory where this script is located
set SCRIPT_DIR=%~dp0
set EXECUTABLE=%SCRIPT_DIR%pidgin.exe

REM Check if the executable exists
if not exist "%EXECUTABLE%" (
    echo Error: pidgin.exe not found in %SCRIPT_DIR%
    echo Please make sure you're running this script from the correct directory.
    pause
    exit /b 1
)

REM Try to install to Program Files (requires admin privileges)
set INSTALL_DIR=%ProgramFiles%\pidgin
echo Attempting to install to: %INSTALL_DIR%

REM Create installation directory
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%" 2>nul
    if errorlevel 1 (
        echo Failed to create installation directory. Trying alternative location...
        set INSTALL_DIR=%USERPROFILE%\AppData\Local\pidgin
        mkdir "%INSTALL_DIR%" 2>nul
        if errorlevel 1 (
            echo Failed to create installation directory: %INSTALL_DIR%
            echo Please run this script as Administrator or choose a different location.
            pause
            exit /b 1
        )
    )
)

REM Copy executable
copy "%EXECUTABLE%" "%INSTALL_DIR%\" >nul
if errorlevel 1 (
    echo Failed to copy executable to %INSTALL_DIR%
    pause
    exit /b 1
)

REM Copy examples
if exist "%SCRIPT_DIR%examples" (
    xcopy "%SCRIPT_DIR%examples" "%INSTALL_DIR%\examples\" /E /I /Y >nul
)

REM Add to PATH if not already there
set PATH_CHECK=0
for /f "tokens=*" %%i in ('echo %PATH%') do (
    echo %%i | findstr /i "%INSTALL_DIR%" >nul
    if not errorlevel 1 set PATH_CHECK=1
)

if %PATH_CHECK%==0 (
    echo.
    echo Installation completed successfully!
    echo.
    echo To use pidgin from anywhere, add this directory to your PATH:
    echo %INSTALL_DIR%
    echo.
    echo You can do this by:
    echo 1. Right-click on "This PC" or "My Computer"
    echo 2. Select "Properties"
    echo 3. Click "Advanced system settings"
    echo 4. Click "Environment Variables"
    echo 5. Under "System variables", find "Path" and click "Edit"
    echo 6. Click "New" and add: %INSTALL_DIR%
    echo 7. Click "OK" on all dialogs
    echo.
    echo After adding to PATH, you can run: pidgin.exe examples\hello.pg
) else (
    echo Installation completed successfully!
    echo pidgin is now available as: pidgin.exe
)

echo.
echo Installation complete!
pause
EOF

    # Note: Update functionality is now integrated into pidgin command
    echo "Update functionality is integrated into pidgin command"
fi

# Create runner scripts
if [ "$OS" = "windows-latest" ]; then
    cat > "pidgin-$PLATFORM/run.bat" << 'EOF'
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
    cat > "pidgin-$PLATFORM/run.sh" << 'EOF'
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
    chmod +x "pidgin-$PLATFORM/run.sh"
fi

# Create README
cat > "pidgin-$PLATFORM/README.md" << EOF
# Pidgin Compiler - $PLATFORM

This is the Pidgin compiler built for $PLATFORM.

## Quick Start

EOF

if [ "$OS" = "windows-latest" ]; then
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
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
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
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

cat >> "pidgin-$PLATFORM/README.md" << 'EOF'

## Examples

Try running some of the included examples:

EOF

if [ "$OS" = "windows-latest" ]; then
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
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
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
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

cat >> "pidgin-$PLATFORM/README.md" << 'EOF'

## Interactive Mode

To start the interactive REPL:

EOF

if [ "$OS" = "windows-latest" ]; then
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
```cmd
run.bat
```
EOF
else
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
```bash
./run.sh
```
EOF
fi

cat >> "pidgin-$PLATFORM/README.md" << 'EOF'

## Installation

EOF

if [ "$OS" = "windows-latest" ]; then
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
### Windows Installation:
You can install the compiler system-wide or use it locally:

#### System-wide installation:
```cmd
install.bat
```

#### Local usage:
```cmd
run.bat examples\hello.pg
```

The installation script will:
- Copy the executable to Program Files (requires admin privileges)
- Fall back to user directory if admin access is not available
- Provide instructions for adding to PATH
- Make it available as `pidgin.exe` command

#### Updating:
```cmd
pidgin.exe update
```
EOF
else
    cat >> "pidgin-$PLATFORM/README.md" << 'EOF'
### Unix-like Installation (Linux, macOS):
You can install the compiler system-wide or use it locally:

#### System-wide installation:
```bash
./install.sh
```

#### Local usage:
```bash
./run.sh examples/hello.pg
```

The installation script will:
- Copy the executable to `/usr/local/bin/` (requires sudo)
- Make it available as `pidgin` command

#### Updating:
```bash
pidgin update
```
EOF
fi

cat >> "pidgin-$PLATFORM/README.md" << 'EOF'

## Language Features

### Basic Types
- **Numbers**: `10`, `3.14`, `-5`
- **Strings**: `"Hello, World!"`
- **Booleans**: `true`, `false`
- **Arrays**: `[1, 2, 3]` (fixed), `{1, 2, 3}` (dynamic)
- **Nil**: `nil` (null value)

### Variables and Assignment
```bash
let x = 10;           # Variable declaration
x = 20;               # Variable assignment
```

### Arithmetic Operations
```bash
let sum = 10 + 5;     # Addition
let diff = 10 - 5;    # Subtraction
let product = 10 * 5; # Multiplication
let quotient = 10 / 5; # Division
```

### Comparisons
```bash
let isEqual = a == b;     # Equality
let isNotEqual = a != b;  # Inequality
let isLess = a < b;       # Less than
let isGreater = a > b;    # Greater than
let isLessEqual = a <= b; # Less than or equal
let isGreaterEqual = a >= b; # Greater than or equal
```

### Control Flow
```bash
# Conditionals
if (x > 5) {
    print "x is greater than 5";
} else {
    print "x is 5 or less";
}

# Loops
while (x > 0) {
    print x;
    x = x - 1;
}
```

### Functions
```bash
function add(a, b) {
    return a + b;
}

let result = add(5, 3);
```

### Arrays and Methods
```bash
let fixed = [1, 2, 3];           # Fixed-size array
let dynamic = {1, 2, 3};         # Dynamic array

let first = fixed[0];            # Array indexing
let len = dynamic.length();      # Get array length
dynamic.push(4);                 # Add element to dynamic array
let last = dynamic.pop();        # Remove and return last element
dynamic.clear();                 # Clear dynamic array
```

### String Operations
```bash
let greeting = "Hello";
let world = "World";
let message = greeting + " " + world;  # String concatenation

# String replacement
let text = "Hello World";
let newText = text.replaceChar`World -> Pidgin`;
```

### Print Statements
```bash
print "Hello, World!";           # Simple print
print "Value: {}", x;            # Format string with one argument
print "Sum: {}, Product: {}", a + b, a * b;  # Multiple arguments
```

### Comments
```bash
// This is a single-line comment
```

### Module Imports
```bash
GET Alpha from math.pg;          # Import single function
GET {Alpha, Beta} from math.pg;  # Import multiple functions
```

## Version Information

**Pidgin Compiler v0.1.15**

### What's New in v0.1.15
- **Enhanced Error Reporting**: Improved line and column information in error messages
- **Array Support**: Added fixed-size arrays `[1, 2, 3]` and dynamic arrays `{1, 2, 3}`
- **Array Methods**: Added `length()`, `push()`, `pop()`, and `clear()` methods
- **String Replacement**: Added `replaceChar` string method with transform syntax
- **Module System**: Enhanced import system with support for multiple imports
- **Better Type System**: Improved type checking and error messages
- **Installation Scripts**: Added `install.sh` for easy installation
- **Update Command**: Added `pidgin update` command for easy updates
- **Windows Support**: Added Windows installation and update scripts

### System Requirements
- **Linux**: x86_64 or ARM64
- **macOS**: Intel (x86_64) or Apple Silicon (ARM64)
- **Windows**: x86_64

### Command Line Options
```bash
pidgin --help          # Show help information
pidgin --version       # Show version information
pidgin file.pg         # Run a Pidgin file
pidgin file.pg --tokens # Show tokens for debugging
pidgin file.pg --ast   # Show AST for debugging
```

### Interactive Mode
Run `pidgin` without arguments to start the interactive REPL:
```bash
pidgin
pidgin> let x = 10;
pidgin> print x;
10
pidgin> exit
```

## Troubleshooting

### Common Issues
1. **Permission Denied**: Make sure the executable has execute permissions
   ```bash
   chmod +x pidgin
   ```

2. **Command Not Found**: Add the installation directory to your PATH
   ```bash
   export PATH="/usr/local/bin:$PATH"  # For system-wide installation
   export PATH="$HOME/.local/bin:$PATH" # For local installation
   ```

3. **File Not Found**: Ensure your `.pg` files exist and have the correct extension

### Getting Help
- **Documentation**: Check the examples directory for sample code
- **Issues**: Report bugs on the GitHub repository
- **Updates**: Use `pidgin update` to get the latest version

## License
This project is open source. See the LICENSE file for details.

---
*Pidgin Compiler - A simple, powerful programming language*
EOF

echo "Distribution created successfully for $PLATFORM" 