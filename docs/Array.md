# Array Implementation in Pidgin Compiler

This document explains the implementation of arrays in the Pidgin programming language, including array creation, indexing, and methods.

## Overview

The Pidgin language supports two types of arrays:
- **Fixed-size arrays**: `[1, 2, 3]` - Immutable size, like Rust arrays
- **Dynamic arrays**: `{1, 2, 3}` - Resizable, like Rust Vec

## Implementation Steps

### 1. AST Structure (`src/ast.rs`)

Added new expression types to represent arrays and array operations:

```rust
pub enum Expr {
    // ... existing variants ...
    FixedArray(Vec<Expr>),    // For [a, b, c]
    DynamicArray(Vec<Expr>),  // For {a, b, c}
    Index {
        array: Box<Expr>,     // The array being indexed
        index: Box<Expr>,     // The index expression
    }, // Array indexing: arr[0]
    // ... existing variants ...
}
```

**Key Points:**
- `FixedArray` and `DynamicArray` store their elements as `Vec<Expr>`
- `Index` expression represents array access like `arr[0]`
- Uses `Box<Expr>` for recursive structure support

### 2. Parser Implementation (`src/parser.rs`)

#### Array Literal Parsing
Updated the `primary()` function to handle array literals:

```rust
Token::LeftBracket => {
    // Parse fixed-size array: [a, b, c]
    let mut elements = Vec::new();
    if !self.check(&Token::RightBracket) {
        loop {
            elements.push(self.expression()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
    }
    self.consume(&Token::RightBracket, "Expect ']' after array elements.")?;
    Ok(Expr::FixedArray(elements))
}
Token::LeftBrace => {
    // Parse dynamic array: {a, b, c}
    // Similar logic but creates DynamicArray
}
```

#### Array Indexing Parsing
Added support for bracket notation after expressions:

```rust
// Check for array indexing
while self.check(&Token::LeftBracket) {
    self.advance(); // consume '['
    let index = self.expression()?; // Parse the index expression
    self.consume(&Token::RightBracket, "Expect ']' after array index.")?;
    
    expr = Expr::Index {
        array: Box::new(expr),
        index: Box::new(index),
    };
}
```

**Key Points:**
- Uses a `while` loop to support chained indexing like `arr[0][1]`
- Index expressions are parsed as regular expressions
- Bracket notation has higher precedence than method calls

#### Method Call Parsing
Extended method call parsing to support array methods:

```rust
let argument = if method_name == "replaceChar" {
    // Special case for replaceChar with backtick syntax
    // ... existing logic ...
} else if method_name == "push" {
    // push method requires an argument
    self.consume(&Token::LeftParen, "Expect '(' after 'push'")?;
    let arg = self.expression()?;
    self.consume(&Token::RightParen, "Expect ')' after push argument")?;
    arg
} else if method_name == "pop" || method_name == "length" || method_name == "clear" {
    // These methods don't take arguments
    self.consume(&Token::LeftParen, "Expect '(' after method name")?;
    self.consume(&Token::RightParen, "Expect ')' after method name")?;
    Expr::Nil // Use Nil as placeholder for no argument
}
```

### 3. Interpreter Implementation (`src/interpreter.rs`)

#### Runtime Values
Added array value types:

```rust
pub enum Value {
    // ... existing variants ...
    FixedArray(Vec<Value>),
    DynamicArray(Vec<Value>),
    // ... existing variants ...
}
```

#### Array Literal Evaluation
```rust
Expr::FixedArray(elements) => {
    let mut vals = Vec::new();
    for e in elements {
        vals.push(self.evaluate_expr(e)?);
    }
    Ok(Value::FixedArray(vals))
}
Expr::DynamicArray(elements) => {
    // Similar logic but creates DynamicArray
}
```

#### Array Indexing Evaluation
```rust
Expr::Index { array, index } => {
    let array_val = self.evaluate_expr(array)?;
    let index_val = self.evaluate_expr(index)?;
    
    let index_num = match index_val {
        Value::Number(n) => n as usize,
        _ => return Err("Array index must be a number".to_string()),
    };
    
    match array_val {
        Value::FixedArray(arr) | Value::DynamicArray(arr) => {
            if index_num >= arr.len() {
                Err(format!("Array index {} out of bounds (array length: {})", index_num, arr.len()))
            } else {
                Ok(arr[index_num].clone())
            }
        }
        _ => Err("Can only index arrays".to_string()),
    }
}
```

#### Array Methods
Implemented four array methods:

1. **`push(value)`** - Add element to dynamic array
2. **`pop()`** - Remove and return last element from dynamic array
3. **`length()`** - Get array length (works on both types)
4. **`clear()`** - Remove all elements from dynamic array

```rust
} else if method == "push" {
    let object_val = self.evaluate_expr(object)?;
    let arg_val = self.evaluate_expr(argument)?;
    
    match object_val {
        Value::DynamicArray(mut arr) => {
            arr.push(arg_val);
            Ok(Value::DynamicArray(arr))
        }
        _ => Err("Push method can only be called on dynamic arrays".to_string()),
    }
}
```

### 4. String Representation
Updated `Value::to_string()` to display arrays properly:

```rust
Value::FixedArray(arr) => {
    let elements = arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ");
    format!("[{}]", elements)
}
Value::DynamicArray(arr) => {
    let elements = arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ");
    format!("{{{}}}", elements)
}
```

## Language Features

### Array Creation
```pidgin
let fixed = [1, 2, 3, 4, 5];      // Fixed-size array
let dynamic = {10, 20, 30};       // Dynamic array
let mixed = [1, "hello", true];   // Mixed types supported
```

### Array Indexing
```pidgin
let first = fixed[0];              // Zero-based indexing
let second = dynamic[1];
let nested = matrix[1][0];         // Nested indexing
```

### Array Methods
```pidgin
// Length (both array types)
let len = fixed.length();
let dyn_len = dynamic.length();

// Dynamic array methods
dynamic = dynamic.push(40);        // Add element
let popped = dynamic.pop();        // Remove and return last element
dynamic = dynamic.clear();         // Remove all elements
```

### Error Handling
- **Index out of bounds**: Returns descriptive error with index and array length
- **Invalid index type**: Only numbers are allowed as indices
- **Method restrictions**: Some methods only work on dynamic arrays
- **Argument validation**: Methods check for correct number of arguments

## Design Decisions

### 1. Syntax Choice
- `[]` for fixed arrays (like Rust arrays)
- `{}` for dynamic arrays (like Rust Vec)
- This provides clear visual distinction between array types

### 2. Method Design
- Methods that modify arrays return new arrays (immutable approach)
- This prevents shared state issues and makes reasoning easier
- Methods are type-safe (e.g., `push` only works on dynamic arrays)

### 3. Error Messages
- Descriptive error messages with context
- Include line/column information where possible
- Clear distinction between different types of errors

### 4. Performance Considerations
- Arrays are stored as `Vec<Value>` for simplicity
- No special optimizations for numeric arrays
- Index bounds checking on every access

## Future Enhancements

1. **Array slicing**: `arr[1:3]` for subarrays
2. **Array concatenation**: `arr1 + arr2`
3. **Array comparison**: `arr1 == arr2`
4. **More methods**: `insert()`, `remove()`, `sort()`
5. **Array literals with expressions**: `[x, y, x + y]`
6. **Multi-dimensional arrays**: Better support for nested arrays

## Testing

The implementation includes comprehensive testing through the `examples/array_test.pg` file, which covers:
- Array creation and display
- Indexing (including bounds checking)
- All available methods
- Error conditions
- Nested arrays

Run tests with:
```bash
cargo run examples/array_test.pg
``` 