# Pidgin Compiler - macos-x86_64

This is the Pidgin compiler built for macos-x86_64.

## Quick Start

### On Unix-like systems (Linux, macOS):
```bash
./run.sh examples/hello.pg
```

### Direct execution:
```bash
./pidgin-compiler examples/hello.pg
```

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

## Installation

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
- Make it available as `pidgin-compiler` command

#### Updating:
```bash
./update.sh
```

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
let newText = text.replaceChar(`World` -> `Pidgin`);
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
- **String Replacement**: Added `replaceChar()` method with transform syntax
- **Module System**: Enhanced import system with support for multiple imports
- **Better Type System**: Improved type checking and error messages
- **Installation Scripts**: Added `install.sh` and `update.sh` for easy installation and updates
- **Windows Support**: Added Windows installation and update scripts

### System Requirements
- **Linux**: x86_64 or ARM64
- **macOS**: Intel (x86_64) or Apple Silicon (ARM64)
- **Windows**: x86_64

### Command Line Options
```bash
pidgin-compiler --help          # Show help information
pidgin-compiler --version       # Show version information
pidgin-compiler file.pg         # Run a Pidgin file
pidgin-compiler file.pg --tokens # Show tokens for debugging
pidgin-compiler file.pg --ast   # Show AST for debugging
```

### Interactive Mode
Run `pidgin-compiler` without arguments to start the interactive REPL:
```bash
pidgin-compiler
pidgin> let x = 10;
pidgin> print x;
10
pidgin> exit
```

## Troubleshooting

### Common Issues
1. **Permission Denied**: Make sure the executable has execute permissions
   ```bash
   chmod +x pidgin-compiler
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
- **Updates**: Use `./update.sh` (Unix) or `update.bat` (Windows) to get the latest version

## License
This project is open source. See the LICENSE file for details.

---
*Pidgin Compiler - A simple, powerful programming language*
