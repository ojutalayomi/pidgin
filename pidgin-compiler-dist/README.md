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
