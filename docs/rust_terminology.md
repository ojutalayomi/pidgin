# Rust Terminology Guide for Pidgin Compiler Development

## Table of Contents
1. [Introduction](#introduction)
2. [Basic Rust Concepts](#basic-rust-concepts)
3. [Lexer/Tokenizer Components](#lexertokenizer-components)
4. [Parser Components](#parser-components)
5. [Abstract Syntax Tree (AST)](#abstract-syntax-tree-ast)
6. [Interpreter Components](#interpreter-components)
7. [Error Handling](#error-handling)
8. [Memory Management](#memory-management)
9. [Pattern Matching](#pattern-matching)
10. [Traits and Implementations](#traits-and-implementations)
11. [Project Structure](#project-structure)

---

## Introduction

This document explains all Rust terminology and concepts used in developing the Pidgin programming language compiler. The Pidgin compiler is a complete interpreter implementation that replaceChares source code through lexical analysis, parsing, and interpretation phases.

---

## Basic Rust Concepts

### `struct`
A custom data type that groups related data together.
```rust
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}
```

### `enum`
A type that can be one of several variants, perfect for representing different token types or AST nodes.
```rust
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    // ... other variants
}
```

### `impl`
Implementation block that defines methods for a struct or enum.
```rust
impl Lexer {
    pub fn new(input: &str) -> Self {
        // Constructor implementation
    }
}
```

### `pub`
Visibility modifier making items public (accessible from other modules).

### `&str` vs `String`
- `&str`: String slice (borrowed reference to string data)
- `String`: Owned string type that can be modified

### `Vec<T>`
Dynamic array (vector) that can grow or shrink at runtime.
```rust
let mut tokens = Vec::new(); // Empty vector
```

### `Option<T>`
Enum representing either `Some(value)` or `None`, used for optional values.
```rust
initializer: Option<Expr> // Can be Some(expression) or None
```

### `Result<T, E>`
Enum for error handling: either `Ok(value)` or `Err(error)`.
```rust
fn parse(&mut self) -> Result<Program, String>
```

---

## Lexer/Tokenizer Components

### Token
Represents the smallest meaningful unit in source code.
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),      // Numeric literals
    String(String),   // String literals
    Identifier(String), // Variable names
    Plus,            // + operator
    Minus,           // - operator
    // ... other tokens
}
```

### TokenInfo
Wrapper struct that combines a token with its position information.
```rust
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}
```

### Lexer
The tokenizer that converts source code into a sequence of tokens.
```rust
pub struct Lexer {
    input: Vec<char>,    // Source code as characters
    position: usize,     // Current position in input
    line: usize,        // Current line number
    column: usize,      // Current column number
}
```

### `advance()`
Method that moves to the next character and updates position tracking.

### `current_char()`
Method that returns the character at the current position.

### `scan_string()`, `scan_number()`, `scan_identifier()`
Specialized methods for parsing different types of tokens.

---

## Parser Components

### Parser
Component that converts tokens into an Abstract Syntax Tree (AST).
```rust
pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
}
```

### Recursive Descent Parsing
Parsing technique where each grammar rule becomes a method:
- `expression()` → `assignment()` → `equality()` → `comparison()` → `term()` → `factor()`

### `peek()`
Method that looks at the current token without consuming it.

### `advance()`
Method that moves to the next token and returns the previous one.

### `consume()`
Method that expects a specific token type and returns an error if not found.

### `match_token()`
Method that checks if the current token matches a given type.

---

## Abstract Syntax Tree (AST)

### Expression (`Expr`)
Represents different types of expressions in the language.
```rust
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Binary { left: Box<Expr>, operator: BinaryOp, right: Box<Expr> },
    Unary { operator: UnaryOp, operand: Box<Expr> },
    Assignment { name: String, value: Box<Expr> },
    MethodCall { object: Box<Expr>, method: String, argument: Box<Expr> },
    Transform { from: String, to: String },
}
```

### Statement (`Stmt`)
Represents different types of statements.
```rust
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    VarDeclaration { name: String, initializer: Option<Expr> },
    Block(Vec<Stmt>),
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { condition: Expr, body: Box<Stmt> },
}
```

### `Box<T>`
Smart pointer for heap allocation, used for recursive data structures.
```rust
left: Box<Expr>  // Heap-allocated expression
```

### BinaryOp and UnaryOp
Enums representing different operators.
```rust
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide,
    Equal, NotEqual, Less, Greater,
    LessEqual, GreaterEqual,
}
```

---

## Interpreter Components

### Value
Runtime values that expressions evaluate to.
```rust
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
```

### Interpreter
The component that executes the AST.
```rust
pub struct Interpreter {
    globals: HashMap<String, Value>,
}
```

### `HashMap<K, V>`
Hash map data structure for storing key-value pairs (variables and their values).

### `execute_stmt()`
Method that executes statements.

### `evaluate_expr()`
Method that evaluates expressions and returns values.

### `is_truthy()`
Method that determines if a value is considered true in conditionals.

---

## Error Handling

### `Result<T, E>`
Rust's standard error handling type.
```rust
fn parse(&mut self) -> Result<Program, String>
```

### `?` Operator
Error propagation operator that returns early if an error occurs.
```rust
let program = parser.parse()?; // Returns error if parsing fails
```

### `panic!()`
Macro that causes the program to crash with an error message.
```rust
panic!("Unexpected character at line {}", line);
```

### `unwrap()` and `expect()`
Methods that extract values from `Result` or `Option`, panicking if there's an error.

---

## Memory Management

### Ownership
Rust's system for managing memory without garbage collection.
- Each value has a single owner
- When owner goes out of scope, value is dropped

### Borrowing
Mechanism for temporarily accessing data without taking ownership.
```rust
fn tokenize(&mut self) -> Vec<TokenInfo>  // Mutable borrow
fn peek(&self) -> &TokenInfo              // Immutable borrow
```

### `&` (Reference)
Allows borrowing without taking ownership.
```rust
&self       // Immutable reference to self
&mut self   // Mutable reference to self
```

### `clone()`
Method that creates a deep copy of data.
```rust
name.clone()  // Creates owned copy of string
```

---

## Pattern Matching

### `match`
Rust's pattern matching construct.
```rust
match token {
    Token::Number(n) => Ok(Expr::Number(n)),
    Token::String(s) => Ok(Expr::String(s.clone())),
    _ => Err("Unexpected token".to_string()),
}
```

### `if let`
Shorthand for matching a single pattern.
```rust
if let Token::Identifier(name) = &token {
    // Use name here
}
```

### `_` (Wildcard)
Matches any value, used as a catch-all.

---

## Traits and Implementations

### `#[derive(...)]`
Attribute that automatically generates trait implementations.
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token { ... }
```

### Common Derived Traits
- `Debug`: Enables `{:?}` formatting for debugging
- `Clone`: Allows creating copies with `.clone()`
- `PartialEq`: Enables equality comparison with `==`

### `Display` vs `Debug`
- `Debug`: For debugging output (`{:?}`)
- `Display`: For user-friendly output (`{}`)

### `to_string()`
Method that converts values to strings.

---

## Project Structure

### `mod`
Module declaration that brings in code from other files.
```rust
mod token;      // Brings in token.rs
mod lexer;      // Brings in lexer.rs
mod parser;     // Brings in parser.rs
```

### `use`
Import statement to bring items into scope.
```rust
use crate::token::{Token, TokenInfo};
use std::collections::HashMap;
```

### `crate`
Refers to the current crate (project).

### `std`
Rust's standard library.

### `pub`
Makes items public so they can be used from other modules.

---

## Cargo and Build System

### `Cargo.toml`
Project configuration file containing metadata and dependencies.

### `cargo build`
Compiles the project.

### `cargo run`
Compiles and runs the project.

### `cargo test`
Runs tests.

### Binary vs Library
- Binary: Executable program with `main()` function
- Library: Reusable code for other projects

---

## Advanced Concepts Used

### Closures
Anonymous functions that can capture variables from their environment.
```rust
|x| x.to_string()  // Simple closure
```

### `Iterator` Methods
- `collect()`: Converts iterator to collection
- `map()`: Transforms each element
- `filter()`: Keeps elements matching condition

### `format!()` Macro
String formatting similar to `printf`.
```rust
format!("Error at line {}", line_number)
```

### `println!()` Macro
Prints to stdout with newline.

### `eprintln!()` Macro
Prints to stderr with newline.

---

## REPL Implementation

### `io::stdin()` and `io::stdout()`
Standard input and output streams.

### `read_line()`
Method to read a line from input.

### `flush()`
Forces output to be written immediately.

### `trim()`
Removes whitespace from strings.

---

## Conclusion

This guide covers all the Rust terminology and concepts used in developing the Pidgin compiler. The project demonstrates fundamental compiler construction techniques while leveraging Rust's powerful type system, memory safety, and pattern matching capabilities.

Key takeaways:
- Rust's enum system is perfect for representing tokens and AST nodes
- The type system helps catch errors at compile time
- Pattern matching makes code clear and exhaustive
- Ownership and borrowing ensure memory safety without garbage collection
- The module system helps organize large projects

The Pidgin compiler serves as an excellent example of how Rust's features align well with compiler development needs.
