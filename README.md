# Pidgin Compiler

A simple programming language compiler built in Rust that supports basic programming constructs and can run any `.pg` file on any computer.

## Quick Start

### Run Programs Instantly

```bash
# Run a program directly
pidgin-compiler examples/hello.pg

# Start interactive mode
pidgin-compiler

# Show debug information
pidgin-compiler --tokens examples/hello.pg
pidgin-compiler --ast examples/hello.pg
```

### Install System-Wide

```bash
# Install so you can use 'pidgin-compiler' from anywhere
./install.sh

# Or install system-wide (requires sudo)
sudo ./install.sh
```

## Features

- **Variables**: `let x = 10;`
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparisons**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Conditionals**: `if`, `else`
- **Loops**: `while`
- **String concatenation**: `"Hello " + "World"`
- **Comments**: `// This is a comment`
- **Print statements**: `print "Hello, World!";`

## Language Grammar

```
program     → statement* EOF
statement   → printStmt | varDecl | ifStmt | whileStmt | block | exprStmt
printStmt   → "print" expression ";"
varDecl     → "let" IDENTIFIER ("=" expression)? ";"
ifStmt      → "if" "(" expression ")" statement ("else" statement)?
whileStmt   → "while" "(" expression ")" statement
block       → "{" statement* "}"
exprStmt    → expression ";"

expression  → assignment
assignment  → IDENTIFIER "=" assignment | equality
equality    → comparison (("==" | "!=") comparison)*
comparison  → term ((">" | ">=" | "<" | "<=") term)*
term        → factor (("-" | "+") factor)*
factor      → unary (("/" | "*") unary)*
unary       → "-" unary | primary
primary     → NUMBER | STRING | "true" | "false" | IDENTIFIER | "(" expression ")"
```

## Usage

### Running a file:
```bash
# Using cargo (development)
cargo run examples/hello.pg

# Using installed compiler (after installation)
pidgin-compiler examples/hello.pg
```

### Interactive REPL:
```bash
# Using cargo (development)
cargo run

# Using installed compiler (after installation)
pidgin-compiler
```

### Debug modes:
```bash
# Show tokens
pidgin-compiler --tokens examples/hello.pg

# Show AST
pidgin-compiler --ast examples/hello.pg
```

## Example Programs

### Hello World
```pidgin
let greeting = "Hello, ";
let name = "World!";
print greeting + name;
```

### Fibonacci Sequence
```pidgin
let n = 10;
let a = 0;
let b = 1;
let count = 0;

print a;
print b;

while (count < n - 2) {
    let next = a + b;
    print next;
    a = b;
    b = next;
    count = count + 1;
}
```

### Conditionals
```pidgin
let x = 15;

if (x > 10) {
    print "x is greater than 10";
} else {
    print "x is not greater than 10";
}
```

## Architecture

The compiler consists of several components:

1. **Lexer** (`lexer.rs`): Tokenizes the source code
2. **Parser** (`parser.rs`): Builds an Abstract Syntax Tree (AST)
3. **AST** (`ast.rs`): Defines the language's syntax tree nodes
4. **Interpreter** (`interpreter.rs`): Executes the AST directly
5. **Tokens** (`token.rs`): Defines all language tokens

## Building and Testing

```bash
# Build the compiler
cargo build

# Build optimized release version
cargo build --release

# Run tests
cargo test

# Run examples
cargo run examples/hello.pg
cargo run examples/fibonacci.pg
cargo run examples/guess.pg
```

## Distribution

### Create Portable Distribution
```bash
# Create distribution for current platform
./distribute.sh

# Create distributions for all platforms
./build-all-platforms.sh
```

### Install System-Wide
```bash
# Interactive installation
./install.sh

# System-wide installation (requires sudo)
sudo ./install.sh
```

## Documentation

- [Installation Guide](INSTALLATION.md) - How to install system-wide
- [Distribution Guide](DISTRIBUTION.md) - How to create portable distributions
- [Portable Usage Guide](PORTABLE_USAGE.md) - How to use portable distributions

## Future Enhancements

- Functions and function calls
- Arrays and objects
- More data types (integers, floats)
- Error recovery in parser
- Code generation to bytecode or native code
- Standard library functions
- File I/O operations
- Memory management optimizations

## Contributing

This is a learning project demonstrating compiler construction in Rust. Feel free to experiment with adding new features or optimizations!
