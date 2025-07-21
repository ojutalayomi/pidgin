# Pidgin Compiler

A modern programming language compiler built in Rust that supports advanced programming constructs including arrays, objects, dates, and method calls. Can run any `.pg` file on any computer with a simple, clean syntax.

## Quick Start

### Run Programs Instantly

```bash
# Run a program directly
pidgin examples/hello.pg

# Start interactive mode
pidgin

# Show debug information
pidgin examples/hello.pg --tokens
pidgin examples/hello.pg --ast
```

### Install System-Wide

```bash
# Install so you can use 'pidgin' from anywhere
./install.sh

# Or install system-wide (requires sudo)
sudo ./install.sh
```

```powershell
# Install so you can use 'pidgin' from anywhere for windows
./install.bat
```

## Features

### Core Language Features
- **Variables**: `let x = 10;`
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparisons**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Conditionals**: `if`, `else`
- **Loops**: `while`
- **String concatenation**: `"Hello " + "World"`
- **Comments**: `// This is a comment`
- **Print statements**: `print "Hello, World!";`
- **Functions**: User-defined functions with parameters and return values

### Advanced Data Types
- **Fixed Arrays**: `[1, 2, 3, 4, 5]` - Immutable arrays with fixed size
- **Dynamic Arrays**: `{1, 2, 3}` - Mutable arrays that can grow/shrink
- **Objects**: `Object()` - Key-value storage containers
- **Dates**: `Date()` - Date and time manipulation
- **Booleans**: `true`, `false`
- **Nil**: `nil` - Represents absence of value

### Array Operations
```pidgin
// Fixed arrays (immutable)
let fixed = [1, 2, 3, 4, 5];
let first = fixed[0];              // Zero-based indexing
let length = fixed.length();       // Get array length

// Dynamic arrays (mutable)
let dynamic = {10, 20, 30};
dynamic = dynamic.push(40);        // Add element
let popped = dynamic.pop();        // Remove and return last element
dynamic = dynamic.clear();         // Remove all elements
```

### String Methods
```pidgin
let text = "Hello World";
let replaced = text.replaceChar(`World->Pidgin`);
// Result: "Hello Pidgin"
```

### Date Operations
```pidgin
let now = Date();                          // Current date/time
let birthday = Date(2024, 1, 15);          // Specific date
let formatted = birthday.format("%Y-%m-%d"); // Format date
let year = birthday.getYear();             // Get year
let month = birthday.getMonth();           // Get month
let day = birthday.getDay();               // Get day
```

### Object Operations
```pidgin
let obj = Object();                        // Create empty object
let person = Object("name", "John", "age", 30);  // Create object with key-value pairs
let config = Object("enabled" => true, "port" => 8080);  // Create object with => syntax
let keys = obj.keys();                     // Get object keys
```

### Built-in Functions
- **`readLine()`**: Read input from console
- **`printErr(message)`**: Print error messages to stderr
- **`Date(...)`**: Create date objects
- **`Object(...)`**: Create object containers with key-value pairs
  - `Object()` - Create empty object
  - `Object("key1", value1, "key2", value2, ...)` - Create object with comma-separated key-value pairs
  - `Object("key1" => value1, "key2" => value2, ...)` - Create object with => syntax

### Module System
```pidgin
// Import single function
GET Add from math.pg;

// Import multiple functions
GET {Add, Multiply, Divide} from math.pg;

// Use imported functions
let result = Add(5, 3);
```

## Language Grammar

```
program     → statement* EOF
statement   → printStmt | varDecl | ifStmt | whileStmt | block | exprStmt | funcDecl | importStmt
printStmt   → "print" expression ("(" expression ("," expression)* ")")? ";"
varDecl     → "let" IDENTIFIER ("=" expression)? ";"
ifStmt      → "if" "(" expression ")" statement ("else" statement)?
whileStmt   → "while" "(" expression ")" statement
block       → "{" statement* "}"
exprStmt    → expression ";"
funcDecl    → "function" IDENTIFIER "(" parameters? ")" block
importStmt  → "GET" importList "from" STRING ";"
importList  → IDENTIFIER | "{" IDENTIFIER ("," IDENTIFIER)* "}"

expression  → assignment
assignment  → IDENTIFIER "=" assignment | equality
equality    → comparison (("==" | "!=") comparison)*
comparison  → term ((">" | ">=" | "<" | "<=") term)*
term        → factor (("-" | "+") factor)*
factor      → unary (("/" | "*") unary)*
unary       → "-" unary | primary
primary     → NUMBER | STRING | "true" | "false" | "nil" | IDENTIFIER | 
              "(" expression ")" | arrayLiteral | objectLiteral | dateLiteral |
              functionCall | methodCall | arrayIndex
arrayLiteral → "[" (expression ("," expression)*)? "]" | "{" (expression ("," expression)*)? "}"
objectLiteral → "Object" "(" ")"
dateLiteral → "Date" "(" (expression ("," expression)*)? ")"
functionCall → IDENTIFIER "(" (expression ("," expression)*)? ")"
methodCall   → expression "." IDENTIFIER methodArgs
methodArgs   → "(" (expression | transform)? ")"
transform    → "`" IDENTIFIER "->" IDENTIFIER "`"
arrayIndex   → expression "[" expression "]"
```

## Usage

### Running a file:
```bash
# Using cargo (development)
cargo run examples/hello.pg

# Using installed compiler (after installation)
pidgin examples/hello.pg
```

### Interactive REPL:
```bash
# Using cargo (development)
cargo run

# Using installed compiler (after installation)
pidgin
```

### Debug modes:
```bash
# Show tokens
pidgin examples/hello.pg --tokens

# Show AST
pidgin examples/hello.pg --ast
```

## Example Programs

### Hello World
```pidgin
let greeting = "Hello, ";
let name = "World!";
print greeting + name;
```

### Array Operations
```pidgin
// Create and manipulate arrays
let numbers = {1, 2, 3, 4, 5};
print "Original array: {}", numbers;

numbers = numbers.push(6);
print "After push: {}", numbers;

let last = numbers.pop();
print "Popped value: {}", last;
print "Final array: {}", numbers;
```

### Date and Time
```pidgin
let now = Date();
print "Current time: {}", now;

let birthday = Date(2024, 1, 15);
let formatted = birthday.format("%Y-%m-%d");
print "Birthday: {}", formatted;

let year = birthday.getYear();
let month = birthday.getMonth();
let day = birthday.getDay();
print "Components: {}/{}/{}", year, month, day;
```

### String Manipulation
```pidgin
let message = "Hello World";
let updated = message.replaceChar(`World->Pidgin`);
print "Original: {}", message;
print "Updated: {}", updated;
```

### Functions
```pidgin
function factorial(n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

let result = factorial(5);
print "5! = {}", result;
```

### Module Import
```pidgin
// math.pg contains: function Add(a, b) { return a + b; }
GET Add from math.pg;
let sum = Add(10, 20);
print "Sum: {}", sum;
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

## Testing

### Comprehensive Test Suite
```bash
# Run comprehensive tests
cargo run examples/comprehensive_test.pg

# Run performance tests
cargo run examples/performance_test.pg

# Run specific feature tests
cargo run examples/array_test.pg
cargo run examples/test_new_features.pg
```

## Architecture

The compiler consists of several components:

1. **Lexer** (`lexer.rs`): Tokenizes the source code with support for all language constructs
2. **Parser** (`parser.rs`): Builds an Abstract Syntax Tree (AST) with method calls and complex expressions
3. **AST** (`ast.rs`): Defines the language's syntax tree nodes including arrays, objects, and methods
4. **Interpreter** (`interpreter.rs`): Executes the AST directly with optimized method dispatch
5. **Tokens** (`token.rs`): Defines all language tokens and their metadata

### Performance Optimizations
- **Single Object Evaluation**: Method calls evaluate the object only once
- **Efficient Method Dispatch**: Uses match statements for fast method resolution
- **Memory Management**: Optimized array and object handling
- **Error Handling**: Comprehensive error messages with line/column information

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
cargo run examples/comprehensive_test.pg
cargo run examples/performance_test.pg
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
- [Array Documentation](docs/Array.md) - Complete array functionality guide
- [Module Import Guide](docs/Module_Import.md) - Module system documentation

## Performance Features

- **Optimized Method Dispatch**: Eliminated redundant object evaluations
- **Efficient Array Operations**: Fast indexing and method calls
- **Memory-Efficient**: Minimal memory overhead for data structures
- **Fast String Operations**: Optimized string manipulation and formatting

## Future Enhancements

- **Standard Library**: Built-in functions for common operations
- **File I/O**: Reading and writing files
- **Error Recovery**: Better error handling and recovery
- **Code Generation**: Compile to bytecode or native code
- **Type System**: Static type checking
- **Packages**: Advanced module and package management
- **Concurrency**: Support for parallel execution
- **Web Assembly**: Compile to WASM for web deployment

## Contributing

This is an actively developed programming language demonstrating modern compiler construction in Rust. Contributions are welcome for:

- New language features
- Performance optimizations
- Documentation improvements
- Test coverage expansion
- Bug fixes and error handling

## License

This project is open source and available under the MIT License.
