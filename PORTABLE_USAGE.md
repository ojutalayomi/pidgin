# Pidgin Compiler - Portable Usage Guide

This guide shows you how to run any `.pg` file on any computer using the portable Pidgin compiler.

## What You Need

1. **The portable distribution** - A zip file containing the Pidgin compiler
2. **Your `.pg` files** - The Pidgin programs you want to run
3. **Basic command line knowledge** - To run the compiler

## Quick Start

### Step 1: Get the Distribution

Download the appropriate distribution for your system:
- **Windows**: `pidgin-windows-x86_64.zip`
- **macOS (Intel)**: `pidgin-macos-x86_64.zip`
- **macOS (Apple Silicon)**: `pidgin-macos-aarch64.zip`
- **Linux (Intel/AMD)**: `pidgin-linux-x86_64.zip`
- **Linux (ARM)**: `pidgin-linux-aarch64.zip`

### Step 2: Extract the Distribution

**On Windows:**
- Right-click the zip file and select "Extract All"
- Or use PowerShell: `Expand-Archive pidgin-windows-x86_64.zip`

**On macOS/Linux:**
```bash
unzip pidgin-macos-x86_64.zip
cd pidgin-macos-x86_64
```

### Step 3: Run Your Programs

**On Windows:**
```cmd
run.bat your-program.pg
```

**On macOS/Linux:**
```bash
./run.sh your-program.pg
```

## Examples

### Running Included Examples

The distribution comes with example programs to test:

**Windows:**
```cmd
run.bat examples\hello.pg
run.bat examples\fibonacci.pg
run.bat examples\simple.pg
```

**macOS/Linux:**
```bash
./run.sh examples/hello.pg
./run.sh examples/fibonacci.pg
./run.sh examples/simple.pg
```

### Running Your Own Programs

1. **Copy your `.pg` file** to the distribution directory
2. **Run it:**

**Windows:**
```cmd
run.bat my-program.pg
```

**macOS/Linux:**
```bash
./run.sh my-program.pg
```

### Interactive Mode

Start the interactive REPL to write and test code:

**Windows:**
```cmd
run.bat
```

**macOS/Linux:**
```bash
./run.sh
```

Then type Pidgin code directly:
```
pidgin> let x = 10;
pidgin> let y = 20;
pidgin> print x + y;
30
pidgin> exit
```

## Language Features

The Pidgin language supports:

- **Variables**: `let x = 10;`
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparisons**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Conditionals**: `if (x > 5) { print "big"; }`
- **Loops**: `while (x < 10) { x = x + 1; }`
- **Strings**: `"Hello, World!"`
- **Comments**: `// This is a comment`
- **Print statements**: `print "Hello";`

## Sample Programs

### Hello World
```pidgin
let greeting = "Hello, ";
let name = "World!";
print greeting + name;
```

### Simple Calculator
```pidgin
let a = 10;
let b = 5;
print "Sum: " + (a + b);
print "Difference: " + (a - b);
print "Product: " + (a * b);
print "Quotient: " + (a / b);
```

### Conditional Example
```pidgin
let age = 18;
if (age >= 18) {
    print "You are an adult";
} else {
    print "You are a minor";
}
```

### Loop Example
```pidgin
let count = 0;
while (count < 5) {
    print "Count: " + count;
    count = count + 1;
}
```

## Troubleshooting

### "Permission denied" error
**macOS/Linux:**
```bash
chmod +x pidgin
chmod +x run.sh
```

### "File not found" error
- Make sure you're in the correct directory
- Check that the file path is correct
- Use relative paths: `./run.sh my-file.pg`

### "Executable not found" error
- Verify the executable exists in the distribution
- Make sure you're using the correct architecture for your system
- Try running it directly: `./pidgin my-file.pg`

### Program doesn't work as expected
- Check the error messages for line and column information
- Verify your syntax matches the examples
- Test with a simple program first

## Advanced Usage

### Running Programs from Different Directories

You can run programs from any directory:

**Windows:**
```cmd
C:\path\to\pidgin\run.bat C:\path\to\my-program.pg
```

**macOS/Linux:**
```bash
/path/to/pidgin/run.sh /path/to/my-program.pg
```

### Using Command Line Arguments

The compiler supports various command line options:

```bash
# Show tokens (for debugging)
./run.sh --tokens my-program.pg

# Show AST (for debugging)
./run.sh --ast my-program.pg

# Run without arguments (interactive mode)
./run.sh
```

### Creating Your Own Distribution

If you want to create a custom distribution with your own programs:

1. **Extract the distribution**
2. **Add your `.pg` files** to the examples directory
3. **Re-zip the directory**
4. **Share the zip file**

## File Structure

Your distribution contains:
```
pidgin-dist/
├── pidgin          # The main executable
├── run.sh                   # Unix/Linux/macOS runner
├── run.bat                  # Windows runner
├── install.sh               # Installation script
├── README.md                # This guide
└── examples/                # Example programs
    ├── hello.pg
    ├── fibonacci.pg
    ├── simple.pg
    └── ...
```

## Getting Help

If you encounter issues:

1. **Check the error messages** - They include line and column information
2. **Try the examples first** - Make sure the basic functionality works
3. **Check file permissions** - Ensure executables have proper permissions
4. **Verify your platform** - Use the correct distribution for your system

## Performance

- The compiler is fast and lightweight
- Programs start almost instantly
- Memory usage is minimal
- No installation or setup required

## Security

- The executable is self-contained with no external dependencies
- It only reads `.pg` files and writes to the console
- No network access or file system modifications
- Safe to run on any system

---

**That's it!** You can now run any `.pg` file on any computer with the portable Pidgin compiler. 