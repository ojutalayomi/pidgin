# Pidgin Compiler - Error Reporting Fix

This document explains how I fixed the issue where error messages in the Pidgin compiler were not showing the correct line and column information.

## Problem Description

The original issue was that error messages always showed "line 1 column 1" regardless of where the error actually occurred in the source code. For example:

```pidgin
let x = 5;
let y = x / "3";  // Error should be at line 2, column 11 (the '/' operator)
print y;
```

Would incorrectly report:
```
Error: Invalid operands for division: Number(5.0) / String("3") at line 1 column 1
```

Instead of the correct:
```
Error: Invalid operands for division: Number(5.0) / String("3") at line 2 column 11
```

## Root Cause Analysis

The problem stemmed from the fact that the Abstract Syntax Tree (AST) didn't store token position information. When the parser created binary expressions (like `x / "3"`), it only stored the operator type (`BinaryOp::Divide`) but not where that operator appeared in the source code.

The interpreter was trying to get position information from the token stream, but this approach was flawed because:

1. The token stream position wasn't being advanced properly during expression evaluation
2. The AST nodes didn't contain the necessary position information
3. The error reporting was happening at the wrong point in the evaluation process

## Solution Overview

The fix involved three main changes:

1. **Modified the AST structure** to include token position information
2. **Updated the parser** to capture and store token positions when creating binary expressions
3. **Updated the interpreter** to use the stored position information for error reporting

## Detailed Implementation

### 1. AST Structure Changes (`src/ast.rs`)

Added line and column fields to the `Expr::Binary` variant:

```rust
Binary {
    left: Box<Expr>,
    operator: BinaryOp,
    right: Box<Expr>,
    line: usize,    // NEW: Line number of the operator
    column: usize,  // NEW: Column number of the operator
},
```

### 2. Parser Changes (`src/parser.rs`)

Updated all four places where binary expressions are created to capture token position information:

**Before:**
```rust
expr = Expr::Binary {
    left: Box::new(expr),
    operator,
    right: Box::new(right),
};
```

**After:**
```rust
let previous_token = self.previous();
expr = Expr::Binary {
    left: Box::new(expr),
    operator,
    right: Box::new(right),
    line: previous_token.line,      // NEW: Store line position
    column: previous_token.column,  // NEW: Store column position
};
```

This change was applied to:
- `equality()` method (for `==` and `!=` operators)
- `comparison()` method (for `<`, `>`, `<=`, `>=` operators)
- `term()` method (for `+` and `-` operators)
- `factor()` method (for `*` and `/` operators)

### 3. Interpreter Changes (`src/interpreter.rs`)

Updated the `evaluate_expr` method to destructure the new position fields and use them in error messages:

**Before:**
```rust
Expr::Binary { left, operator, right } => {
    // ... evaluation logic ...
    _ => {
        let (line, column) = self.current_position();
        Err(format!("Invalid operands for division: {:?} / {:?} at line {} column {}", left_val, right_val, line, column))
    }
}
```

**After:**
```rust
Expr::Binary { left, operator, right, line, column } => {
    // ... evaluation logic ...
    _ => {
        Err(format!("Invalid operands for division: {:?} / {:?} at line {} column {}", left_val, right_val, line, column))
    }
}
```

## Testing the Fix

### Test Case 1: Division Error
```pidgin
let x = 5;
let y = x / "3";
print y;
```

**Result:**
```
Error: Invalid operands for division: Number(5.0) / String("3") at line 2 column 11
```

### Test Case 2: Working Code
```pidgin
let x = 10;
let y = 20;
let sum = x + y;
print sum;
```

**Result:**
```
30
```

The fix correctly reports the position of the division operator `/` at line 2, column 11, while not affecting the execution of valid code.

## Key Insights

1. **Token Position Preservation**: The key insight was that token position information needs to be preserved in the AST, not retrieved during interpretation.

2. **Parser Responsibility**: The parser is the component that has access to token position information, so it should be responsible for storing this information in the AST.

3. **Error Location Accuracy**: By storing position information at parse time, we ensure that error messages point to the exact location where the problematic operation occurs.

## Benefits of This Approach

1. **Accurate Error Reporting**: Error messages now show the exact line and column where errors occur
2. **Better Debugging Experience**: Users can quickly locate and fix issues in their code
3. **Maintainable Code**: The solution is clean and doesn't require complex token stream management in the interpreter
4. **Extensible**: This pattern can be extended to other AST nodes that need position information

## Files Modified

- `src/ast.rs`: Added line and column fields to Binary expression
- `src/parser.rs`: Updated binary expression creation to capture token positions
- `src/interpreter.rs`: Updated error reporting to use stored position information

## Conclusion

This fix demonstrates the importance of preserving source location information during the parsing phase and using it during error reporting. The solution is elegant, maintainable, and provides users with accurate error messages that significantly improve the debugging experience.
