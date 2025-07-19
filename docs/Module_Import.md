# Module Import Implementation in Pidgin Compiler

This document explains the implementation of module imports using the `GET` keyword in the Pidgin programming language.

## Overview

The Pidgin language supports module imports using the `GET` keyword, allowing you to import functions and variables from other `.pg` files. The import system enforces case-sensitive visibility rules where only names starting with uppercase letters can be imported.

## Syntax

### Single Import
```pidgin
GET Alpha from math.pg;
GET Beta from utils.pg;
```

### Multiple Import
```pidgin
GET {Alpha, Beta, Gamma} from math.pg;
GET {Function1, Variable1, Function2} from module.pg;
```

### Alternative Syntax
```pidgin
GET Alpha GET math.pg;  // Alternative to "from"
GET {Alpha, Beta} GET module.pg;
```

## Implementation Steps

### 1. Lexer Changes (`src/lexer.rs`)

Added the `Get` token and updated keyword recognition:

```rust
// In Token enum
Get, // 'get' keyword token for module imports

// In keyword_or_identifier function
"get" => Token::Get,   // get keyword for imports
```

**Key Points:**
- Case-insensitive matching for the `GET` keyword
- Supports both `GET` and `get` in source code

### 2. AST Changes (`src/ast.rs`)

Added the `Import` statement to the AST:

```rust
pub enum Stmt {
    // ... existing variants ...
    Import {
        names: Vec<String>,     // Names to import (can be single or multiple)
        module: String,         // Module file path
    }, // Import statement: GET Alpha from math.pg;
    // ... existing variants ...
}
```

**Key Points:**
- `names` is a vector to support both single and multiple imports
- `module` stores the full module path (e.g., "math.pg")

### 3. Parser Changes (`src/parser.rs`)

#### Import Statement Recognition
Updated the `statement()` function to recognize `GET` statements:

```rust
fn statement(&mut self) -> Result<Stmt, String> {
    if self.match_token(&Token::Get) { // Check for import statement
        return self.import_statement(); // Parse import statement
    }
    // ... rest of statement parsing ...
}
```

#### Import Statement Parsing
Added the `import_statement()` function to parse both single and multiple imports:

```rust
fn import_statement(&mut self) -> Result<Stmt, String> {
    let mut names = Vec::new();
    
    // Parse the names to import
    if self.match_token(&Token::LeftBrace) {
        // Multiple names: GET {Alpha,B} from math.pg;
        loop {
            let name_token = self.consume_identifier("Expect identifier in import list")?;
            // ... parse names ...
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        self.consume(&Token::RightBrace, "Expect '}' after import list")?;
    } else {
        // Single name: GET Alpha from math.pg;
        let name_token = self.consume_identifier("Expect identifier after GET")?;
        // ... parse single name ...
    }
    
    // Parse "from" or "GET" keyword
    // Parse module path with dot notation support
    // ... rest of parsing logic ...
}
```

**Key Features:**
- Supports both single and multiple imports
- Handles dot notation in module paths (e.g., "math.pg")
- Accepts both "from" and "GET" as the separator keyword
- Proper error messages for syntax errors

### 4. Interpreter Changes (`src/interpreter.rs`)

#### Import Statement Execution
Added import handling to `execute_stmt()`:

```rust
Stmt::Import { names, module } => {
    self.load_module(names, module)?;
    Ok(ControlFlow::None)
}
```

#### Module Loading
Implemented the `load_module()` method:

```rust
fn load_module(&mut self, names: &[String], module_path: &str) -> Result<(), String> {
    // 1. Find the module file
    let module_file = self.find_module_file(module_path)?;
    
    // 2. Read and parse the module
    let source = fs::read_to_string(&module_file)?;
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    // 3. Execute the module in isolation
    let mut module_interpreter = Interpreter::new(None);
    for stmt in program.statements {
        module_interpreter.execute_stmt(&stmt)?;
    }
    
    // 4. Import requested names (only uppercase)
    for name in names {
        if let Some(value) = module_interpreter.globals.get(name) {
            if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                self.globals.insert(name.clone(), value.clone());
            } else {
                return Err(format!("Cannot import '{}' - only names starting with uppercase letters can be imported", name));
            }
        } else {
            return Err(format!("Name '{}' not found in module '{}'", name, module_file));
        }
    }
    
    Ok(())
}
```

**Key Features:**
- Module isolation (each module has its own interpreter)
- Case-sensitive import rules
- Automatic file extension handling
- Relative path resolution
- Comprehensive error handling

## Language Features

### Import Rules
1. **Case Sensitivity**: Only names starting with uppercase letters can be imported
2. **Module Isolation**: Each module executes in its own scope
3. **File Extension**: `.pg` extension is automatically added if missing
4. **Path Resolution**: Supports relative paths and looks in common directories

### Example Module (`math.pg`)
```pidgin
// Exportable (uppercase)
function Alpha() {
    return 1;
}

function Beta(x, y) {
    return x + y;
}

let B = "a boy";
let C = 42;

// Not exportable (lowercase)
function alphabet() {
    return "lowercase function";
}

let x = 10;
```

### Example Usage (`main.pg`)
```pidgin
// Import single items
GET Alpha from math.pg;
GET B from math.pg;

// Import multiple items
GET {Beta, C} from math.pg;

// Use imported functions and variables
print "Alpha(): {}", Alpha();
print "B: {}", B;
print "Beta(5, 3): {}", Beta(5, 3);
```

## Error Handling

### Import Errors
- **Module not found**: Clear error message with attempted paths
- **Name not found**: Specifies which name is missing from which module
- **Case violation**: Explains the uppercase requirement for imported names
- **Syntax errors**: Detailed error messages for malformed import statements

### Error Examples
```bash
# Module not found
Error: Module 'nonexistent.pg' not found. Tried: nonexistent.pg, examples/nonexistent.pg

# Case violation
Error: Cannot import 'alphabet' - only names starting with uppercase letters can be imported

# Name not found
Error: Name 'MissingFunction' not found in module 'math.pg'
```

## Design Decisions

### 1. Case-Sensitive Import Rules
- **Rationale**: Provides clear visibility control without explicit export keywords
- **Implementation**: Checks first character of imported names
- **Benefits**: Simple, intuitive, and prevents accidental imports

### 2. Module Isolation
- **Rationale**: Prevents naming conflicts and provides clean separation
- **Implementation**: Each module gets its own interpreter instance
- **Benefits**: Predictable behavior and easier debugging

### 3. Path Resolution
- **Rationale**: Makes imports work regardless of current working directory
- **Implementation**: Tries multiple paths including examples directory
- **Benefits**: More user-friendly and flexible

### 4. Alternative Syntax Support
- **Rationale**: Provides flexibility in import syntax
- **Implementation**: Accepts both "from" and "GET" as separators
- **Benefits**: Accommodates different coding styles

## Testing

### Test Files Created
1. **`examples/math.pg`** - Module with exportable and non-exportable items
2. **`examples/main.pg`** - Main program demonstrating imports
3. **`examples/import_error_test.pg`** - Error handling tests

### Test Cases Covered
- ✅ Single import functionality
- ✅ Multiple import functionality
- ✅ Case-sensitive import rules
- ✅ Module file resolution
- ✅ Error handling for missing modules
- ✅ Error handling for invalid names
- ✅ Function and variable imports
- ✅ Usage of imported items

### Running Tests
```bash
# Test successful imports
cargo run examples/main.pg

# Test error handling
cargo run examples/import_error_test.pg
```

## Performance Considerations

### Module Loading
- **Lazy Loading**: Modules are loaded only when imported
- **Caching**: No module caching implemented (each import reloads)
- **Memory**: Each module creates a separate interpreter instance

### Future Optimizations
1. **Module Caching**: Cache loaded modules to avoid re-parsing
2. **Circular Import Detection**: Prevent infinite import loops
3. **Selective Loading**: Only load requested symbols from modules

## Limitations and Future Enhancements

### Current Limitations
1. **No Circular Imports**: No detection or handling of circular dependencies
2. **No Module Caching**: Modules are reloaded on each import
3. **Simple Path Resolution**: Limited to current directory and examples/
4. **No Namespace Support**: All imports go to global scope

### Future Enhancements
1. **Module Aliases**: `GET Alpha as MyAlpha from math.pg;`
2. **Wildcard Imports**: `GET * from math.pg;`
3. **Nested Module Support**: `GET Alpha from utils/math.pg;`
4. **Export Keywords**: Explicit export declarations
5. **Module Caching**: Performance improvements
6. **Circular Import Detection**: Better dependency management

## Integration with Existing Features

### Compatibility
- **Arrays**: Imported functions can work with arrays
- **String Methods**: Imported functions can use string methods
- **Functions**: Full function import and usage support
- **Variables**: All variable types can be imported

### Example Integration
```pidgin
// math.pg
function ProcessArray(arr) {
    return arr.length();
}

// main.pg
GET ProcessArray from math.pg;
let myArray = [1, 2, 3];
print "Array length: {}", ProcessArray(myArray);
```

## Summary

The module import system provides a clean, simple way to organize Pidgin code across multiple files. The case-sensitive import rules ensure clear visibility control, while the isolated module execution prevents naming conflicts. The implementation is robust with comprehensive error handling and supports both single and multiple imports with flexible syntax.

The system successfully integrates with all existing language features and provides a solid foundation for future enhancements like module caching, circular import detection, and more advanced import syntax.