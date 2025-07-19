use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp}; // Import AST types
use crate::token::TokenInfo; // Import necessary types from the Token module
use chrono::{DateTime, Datelike, Local};
use std::collections::HashMap; // Import HashMap for variable storage
use std::fmt;
use std::io::{self, Write};

// Define a custom result type for handling returns
#[derive(Debug, Clone)]
pub enum ControlFlow {
    None,
    Return(Value),
}

// Define the Value enum, representing all possible runtime values
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),    // Numeric value
    String(String), // String value
    Boolean(bool),  // Boolean value
    FixedArray(Vec<Value>),
    DynamicArray(Vec<Value>),
    Object(HashMap<String, Value>),   // Object with key-value pairs
    Date(DateTime<Local>),            // Date/time value
    Nil,                              // Nil (no value)
    Function(Vec<String>, Box<Stmt>), // Function value
}

// Implement Display trait for Value
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"), // Convert number to string
            Value::String(s) => write!(f, "{s}"), // Clone string
            Value::Boolean(b) => write!(f, "{b}"), // Convert bool to string
            Value::Nil => write!(f, "nil"),       // Nil as "nil"
            Value::Function(params, _body) => {
                let params_str = params.join(", ");
                write!(f, "function({params_str}) {{ ... }}")
            }
            Value::FixedArray(arr) => {
                let elements = arr
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{elements}]")
            }
            Value::DynamicArray(arr) => {
                let elements = arr
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{{elements}}}")
            }
            Value::Object(obj) => {
                let mut pairs = Vec::new();
                for (key, value) in obj {
                    pairs.push(format!("{key}: {value}"));
                }
                write!(f, "{{ {} }}", pairs.join(", "))
            }
            Value::Date(dt) => {
                write!(f, "{}", dt.format("%Y-%m-%d %H:%M:%S"))
            }
        }
    }
}

// Implement methods for Value
impl Value {
    // Check if the value is truthy (for conditionals)
    fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b, // Boolean: use its value
            Value::Nil => false,     // Nil is always false
            _ => true,               // All other values are truthy
        }
    }

    // Optimized equality check
    fn is_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::FixedArray(a), Value::FixedArray(b))
            | (Value::DynamicArray(a), Value::DynamicArray(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(x, y)| x.is_equal(y))
            }
            _ => false,
        }
    }
}

// Define the Interpreter struct, which executes the AST
pub struct Interpreter {
    globals: HashMap<String, Value>, // Store global variables
    tokens: Option<Vec<TokenInfo>>,
    current: usize, // Current position in the token stream
}

// Implement methods for Interpreter
impl Interpreter {
    // Create a new Interpreter
    pub fn new(tokens: Option<Vec<TokenInfo>>) -> Self {
        Self {
            globals: HashMap::new(), // Start with empty globals
            tokens: Some(tokens.unwrap_or_default()),
            current: 0, // Start at the first token
        }
    }

    // Interpret a program (execute all statements)
    pub fn interpret(&mut self, program: Program, tokens: Vec<TokenInfo>) -> Result<(), String> {
        self.tokens = Some(tokens);
        self.current = 0; // Reset to the beginning of the token stream
        for statement in program.statements {
            // Loop through all statements
            match self.execute_stmt(&statement)? {
                // Execute each statement
                ControlFlow::Return(_) => {
                    return Err("Return statement not allowed outside function".to_string());
                }
                ControlFlow::None => continue,
            }
        }
        Ok(()) // Return Ok if all statements executed
    }

    // Execute a statement
    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<ControlFlow, String> {
        match stmt {
            Stmt::Return(expr) => {
                let value = self.evaluate_expr(expr)?;
                Ok(ControlFlow::Return(value))
            }
            Stmt::Expression(expr) => {
                self.evaluate_expr(expr)?; // Evaluate the expression
                Ok(ControlFlow::None) // No value to return
            }
            Stmt::Print { format, arguments } => {
                let format_value = self.evaluate_expr(format)?;

                if arguments.is_empty() {
                    // Simple print: print value;
                    println!("{format_value}");
                } else {
                    // Format string print: print "{}", value;
                    let format_str = match format_value {
                        Value::String(s) => s,
                        _ => return Err("Format string must be a string".to_string()),
                    };

                    // Evaluate all arguments
                    let arg_values: Vec<String> = arguments
                        .iter()
                        .map(|arg| self.evaluate_expr(arg).map(|v| v.to_string()))
                        .collect::<Result<_, _>>()?;

                    // Replace each '{}' in format_str with the corresponding argument
                    let mut formatted = String::new();
                    let mut parts = format_str.split("{}");
                    let mut args_iter = arg_values.iter();

                    if let Some(first) = parts.next() {
                        formatted.push_str(first);
                    }
                    for part in parts {
                        if let Some(arg) = args_iter.next() {
                            formatted.push_str(arg);
                        } else {
                            formatted.push_str("{}"); // Not enough arguments, keep as is
                        }
                        formatted.push_str(part);
                    }
                    // If there are extra arguments, ignore them

                    println!("{formatted}"); // Print the value
                }
                Ok(ControlFlow::None)
            }
            Stmt::VarDeclaration { name, initializer } => {
                let value = if let Some(init) = initializer {
                    self.evaluate_expr(init)? // Evaluate initializer if present
                } else {
                    Value::Nil // Otherwise, use Nil
                };
                self.globals.insert(name.clone(), value); // Store variable in globals
                Ok(ControlFlow::None)
            }
            Stmt::FunctionDeclaration {
                name,
                parameters,
                body,
            } => {
                let function_value = Value::Function(parameters.clone(), body.clone()); // Create function value
                self.globals.insert(name.clone(), function_value); // Store function in globals
                Ok(ControlFlow::None)
            }
            Stmt::Import { names, module } => {
                self.load_module(names, module)?;
                Ok(ControlFlow::None)
            }
            Stmt::Block(statements) => {
                for stmt in statements {
                    match self.execute_stmt(stmt)? {
                        // Execute each statement in the block
                        ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                        ControlFlow::None => continue,
                    }
                }
                Ok(ControlFlow::None)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.evaluate_expr(condition)?; // Evaluate the condition

                if condition_value.is_truthy() {
                    self.execute_stmt(then_branch) // Execute then branch if true
                } else if let Some(else_stmt) = else_branch {
                    self.execute_stmt(else_stmt) // Execute else branch if present
                } else {
                    Ok(ControlFlow::None)
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    let condition_value = self.evaluate_expr(condition)?; // Evaluate the condition
                    if !condition_value.is_truthy() {
                        break; // Exit loop if condition is false
                    }
                    match self.execute_stmt(body)? {
                        // Execute loop body
                        ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                        ControlFlow::None => continue,
                    }
                }
                Ok(ControlFlow::None)
            }
        }
    }

    // Evaluate an expression and return its value
    fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)), // Numeric literal
            Expr::String(s) => Ok(Value::String(s.clone())), // String literal
            Expr::Boolean(b) => Ok(Value::Boolean(*b)), // Boolean literal
            Expr::Identifier(name) => {
                if let Some(value) = self.globals.get(name) {
                    Ok(value.clone()) // Return variable value if found
                } else {
                    Err(format!("Undefined variable '{name}'")) // Error if not found
                }
            }
            Expr::FixedArray(elements) => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.evaluate_expr(e)?);
                }
                Ok(Value::FixedArray(vals))
            }
            Expr::DynamicArray(elements) => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.evaluate_expr(e)?);
                }
                Ok(Value::DynamicArray(vals))
            }
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
                            Err(format!(
                                "Array index {index_num} out of bounds (array length: {})",
                                arr.len()
                            ))
                        } else {
                            Ok(arr[index_num].clone())
                        }
                    }
                    _ => Err("Can only index arrays".to_string()),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
                line,
                column,
            } => {
                let left_val = &self.evaluate_expr(left)?; // Evaluate left operand
                let right_val = &self.evaluate_expr(right)?; // Evaluate right operand

                match operator {
                    BinaryOp::Add => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)), // Add numbers
                        (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))), // Concatenate strings
                        (Value::String(a), Value::Number(b)) => Ok(Value::String(format!("{a}{b}"))), // String + number
                        (Value::Number(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))), // Number + string
                        (Value::String(a), Value::Boolean(b)) => Ok(Value::String(format!("{a}{b}"))), // String + bool
                        (Value::Boolean(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))), // Bool + string
                        _ => {
                            Err(format!("Invalid operands for addition: {left_val:?} + {right_val:?} at line {line} column {column}"))
                        }
                    },
                    BinaryOp::Subtract => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)), // Subtract numbers
                        _ => {
                            Err(format!("Invalid operands for subtraction: {left_val:?} - {right_val:?} at line {line} column {column}"))
                        }
                    },
                    BinaryOp::Multiply => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)), // Multiply numbers
                        _ => {
                            Err(format!("Invalid operands for multiplication: {left_val:?} * {right_val:?} at line {line} column {column}"))
                        }
                    },
                    BinaryOp::Divide => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => {
                            if *b == 0.0 {
                                Err("Division by zero".to_string()) // Error for division by zero
                            } else {
                                Ok(Value::Number(a / b)) // Divide numbers
                            }
                        }
                        _ => {
                            Err(format!("Invalid operands for division: {left_val:?} / {right_val:?} at line {line} column {column}"))
                        }
                    },
                    BinaryOp::Equal => Ok(Value::Boolean(left_val.is_equal(right_val))), // Equality check
                    BinaryOp::NotEqual => Ok(Value::Boolean(!left_val.is_equal(right_val))), // Not-equal check
                    BinaryOp::Greater => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)), // Greater than
                        _ => {
                            Err(format!("Invalid operands for comparison: {left_val:?} > {right_val:?} at line {line} column {column}"))
                        }
                    },
                    BinaryOp::GreaterEqual => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)), // Greater or equal
                        _ => {
                            Err(format!("Invalid operands for comparison: {left_val:?} >= {right_val:?} at line {line} column {column}"))
                        }
                    },
                    BinaryOp::Less => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)), // Less than
                        _ => {
                            Err(format!("Invalid operands for comparison: {left_val:?} < {right_val:?} at line {line} column {column}"))
                        }
                    },
                    BinaryOp::LessEqual => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)), // Less or equal
                        _ => {
                            Err(format!("Invalid operands for comparison: {left_val:?} <= {right_val:?} at line {line} column {column}"))
                        }
                    },
                }
            }
            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate_expr(operand)?; // Evaluate operand

                match operator {
                    UnaryOp::Minus => match operand_val {
                        Value::Number(n) => Ok(Value::Number(-n)), // Negate number
                        _ => Err("Invalid operand for unary minus".to_string()), // Error for invalid type
                    },
                }
            }
            Expr::Assignment { name, value } => {
                let val = self.evaluate_expr(value)?; // Evaluate right-hand side
                self.globals.insert(name.clone(), val.clone()); // Assign to variable
                Ok(val) // Return the value
            }
            Expr::MethodCall {
                object,
                method,
                argument,
            } => {
                // Evaluate object once at the beginning
                let object_val = self.evaluate_expr(object)?;

                // Remove debug print in production
                // eprintln!("DEBUG: Method '{}' called on object type: {:?}", method, std::mem::discriminant(&object_val));

                match method.as_str() {
                    "replaceChar" => {
                        if let Value::String(original) = &object_val {
                            if let Expr::Transform { from, to } = argument.as_ref() {
                                // Try to resolve 'from' as a variable, fallback to literal if not found
                                let from_value = if let Some(val) = self.globals.get(from) {
                                    match val {
                                        Value::String(s) => s.clone(),
                                        Value::Number(n) => n.to_string(),
                                        Value::Boolean(b) => b.to_string(),
                                        _ => {
                                            return Err(format!(
                                                "Variable '{from}' is not a valid replacement value"
                                            ))
                                        }
                                    }
                                } else {
                                    from.clone() // Use as literal if not a variable
                                };
                                // Try to resolve 'to' as a variable, fallback to literal if not found
                                let to_value = if let Some(val) = self.globals.get(to) {
                                    match val {
                                        Value::String(s) => s.clone(),
                                        Value::Number(n) => n.to_string(),
                                        Value::Boolean(b) => b.to_string(),
                                        _ => {
                                            return Err(format!(
                                                "Variable '{to}' is not a valid replacement value"
                                            ))
                                        }
                                    }
                                } else {
                                    to.clone() // Use as literal if not a variable
                                };
                                let result = original.replace(&from_value, &to_value);
                                Ok(Value::String(result))
                            } else {
                                Err("ReplaceChar method requires a transform argument".to_string())
                            }
                        } else {
                            Err("ReplaceChar method can only be called on strings".to_string())
                        }
                    }
                    "push" => {
                        // Array push method: arr.push(value)
                        let arg_val = self.evaluate_expr(argument)?;

                        if let Value::DynamicArray(mut arr) = object_val {
                            arr.push(arg_val);
                            Ok(Value::DynamicArray(arr))
                        } else {
                            Err("Push method can only be called on dynamic arrays".to_string())
                        }
                    }
                    "pop" => {
                        // Array pop method: arr.pop()
                        // Verify no argument was provided
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::DynamicArray(mut arr) = object_val {
                                if arr.is_empty() {
                                    Err("Cannot pop from empty array".to_string())
                                } else {
                                    let popped = arr.pop().unwrap();
                                    Ok(popped)
                                }
                            } else {
                                Err("Pop method can only be called on dynamic arrays".to_string())
                            }
                        } else {
                            Err("Pop method does not take arguments".to_string())
                        }
                    }
                    "length" => {
                        // Array length method: arr.length()
                        // Verify no argument was provided
                        if let Expr::Nil = argument.as_ref() {
                            match object_val {
                                Value::FixedArray(arr) | Value::DynamicArray(arr) => {
                                    Ok(Value::Number(arr.len() as f64))
                                }
                                _ => Err("Length method can only be called on arrays".to_string()),
                            }
                        } else {
                            Err("Length method does not take arguments".to_string())
                        }
                    }
                    "clear" => {
                        // Array clear method: arr.clear()
                        // Verify no argument was provided
                        if let Expr::Nil = argument.as_ref() {
                            match object_val {
                                Value::DynamicArray(_) => Ok(Value::DynamicArray(Vec::new())),
                                _ => {
                                    Err("Clear method can only be called on dynamic arrays"
                                        .to_string())
                                }
                            }
                        } else {
                            Err("Clear method does not take arguments".to_string())
                        }
                    }
                    "format" => {
                        // Date format method: date.format("%Y-%m-%d")
                        if let Value::Date(dt) = object_val {
                            let fmt_val = self.evaluate_expr(argument)?;
                            if let Value::String(fmt) = fmt_val {
                                Ok(Value::String(dt.format(&fmt).to_string()))
                            } else {
                                Err("Date.format() requires a string argument".to_string())
                            }
                        } else {
                            Err("format method can only be called on Date objects".to_string())
                        }
                    }
                    "getYear" => {
                        // Date getYear method: date.getYear()
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::Date(dt) = object_val {
                                Ok(Value::Number(dt.year() as f64))
                            } else {
                                Err("getYear method can only be called on Date objects".to_string())
                            }
                        } else {
                            Err("getYear method does not take arguments".to_string())
                        }
                    }
                    "getMonth" => {
                        // Date getMonth method: date.getMonth()
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::Date(dt) = object_val {
                                Ok(Value::Number(dt.month() as f64))
                            } else {
                                Err("getMonth method can only be called on Date objects"
                                    .to_string())
                            }
                        } else {
                            Err("getMonth method does not take arguments".to_string())
                        }
                    }
                    "getDay" => {
                        // Date getDay method: date.getDay()
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::Date(dt) = object_val {
                                Ok(Value::Number(dt.day() as f64))
                            } else {
                                Err("getDay method can only be called on Date objects".to_string())
                            }
                        } else {
                            Err("getDay method does not take arguments".to_string())
                        }
                    }
                    "keys" => {
                        // Object keys method: obj.keys()
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::Object(obj) = object_val {
                                let keys = obj.keys().map(|k| Value::String(k.clone())).collect();
                                Ok(Value::DynamicArray(keys))
                            } else {
                                Err("keys method can only be called on Object".to_string())
                            }
                        } else {
                            Err("keys method does not take arguments".to_string())
                        }
                    }
                    "insert" => {
                        // Array insert method: arr.insert(index, value)
                        if let Value::DynamicArray(mut arr) = object_val {
                            if let Expr::Binary {
                                left,
                                operator: _,
                                right,
                                ..
                            } = argument.as_ref()
                            {
                                if let (Expr::Number(index), _) = (left.as_ref(), right.as_ref()) {
                                    let index = *index as usize;
                                    if index > arr.len() {
                                        return Err(format!(
                                            "Insert index {index} out of bounds (array length: {})",
                                            arr.len()
                                        ));
                                    }
                                    let value = self.evaluate_expr(right)?;
                                    arr.insert(index, value);
                                    Ok(Value::DynamicArray(arr))
                                } else {
                                    Err("insert() requires (index, value) arguments".to_string())
                                }
                            } else {
                                Err("insert() requires exactly two arguments".to_string())
                            }
                        } else {
                            Err("insert method can only be called on dynamic arrays".to_string())
                        }
                    }
                    "remove" => {
                        // Array remove method: arr.remove(index)
                        if let Value::DynamicArray(mut arr) = object_val {
                            if let Expr::Number(index) = argument.as_ref() {
                                let index = *index as usize;
                                if index >= arr.len() {
                                    return Err(format!(
                                        "Remove index {index} out of bounds (array length: {})",
                                        arr.len()
                                    ));
                                }
                                let removed = arr.remove(index);
                                Ok(removed)
                            } else {
                                Err("remove() requires a numeric index argument".to_string())
                            }
                        } else {
                            Err("remove method can only be called on dynamic arrays".to_string())
                        }
                    }
                    "reverse" => {
                        // Array reverse method: arr.reverse()
                        if let Expr::Nil = argument.as_ref() {
                            match object_val {
                                Value::DynamicArray(mut arr) => {
                                    arr.reverse();
                                    Ok(Value::DynamicArray(arr))
                                }
                                Value::FixedArray(mut arr) => {
                                    arr.reverse();
                                    Ok(Value::FixedArray(arr))
                                }
                                _ => Err("reverse method can only be called on arrays".to_string()),
                            }
                        } else {
                            Err("reverse method does not take arguments".to_string())
                        }
                    }
                    "toUpper" => {
                        // String toUpper method: str.toUpper()
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::String(s) = object_val {
                                Ok(Value::String(s.to_uppercase()))
                            } else {
                                Err("toUpper method can only be called on strings".to_string())
                            }
                        } else {
                            Err("toUpper method does not take arguments".to_string())
                        }
                    }
                    "toLower" => {
                        // String toLower method: str.toLower()
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::String(s) = object_val {
                                Ok(Value::String(s.to_lowercase()))
                            } else {
                                Err("toLower method can only be called on strings".to_string())
                            }
                        } else {
                            Err("toLower method does not take arguments".to_string())
                        }
                    }
                    "trim" => {
                        // String trim method: str.trim()
                        if let Expr::Nil = argument.as_ref() {
                            if let Value::String(s) = object_val {
                                Ok(Value::String(s.trim().to_string()))
                            } else {
                                Err("trim method can only be called on strings".to_string())
                            }
                        } else {
                            Err("trim method does not take arguments".to_string())
                        }
                    }
                    "set" => {
                        // Object set method: obj.set(key, value)
                        if let Value::Object(mut obj) = object_val {
                            if let Expr::Binary {
                                left,
                                operator: _,
                                right,
                                ..
                            } = argument.as_ref()
                            {
                                // Evaluate the key to get its string value
                                let key_val = self.evaluate_expr(left)?;
                                let key = match key_val {
                                    Value::String(s) => s,
                                    _ => return Err("set() requires (key, value) arguments where key is a string".to_string()),
                                };
                                let value = self.evaluate_expr(right)?;
                                obj.insert(key, value);
                                Ok(Value::Object(obj))
                            } else {
                                Err("set() requires exactly two arguments".to_string())
                            }
                        } else {
                            Err("set method can only be called on objects".to_string())
                        }
                    }
                    "get" => {
                        // Object get method: obj.get(key)
                        if let Value::Object(obj) = object_val {
                            let key_val = self.evaluate_expr(argument)?;
                            let key = match key_val {
                                Value::String(s) => s,
                                _ => return Err("get() requires a string key argument".to_string()),
                            };
                            if let Some(value) = obj.get(&key) {
                                Ok(value.clone())
                            } else {
                                Ok(Value::Nil) // Return nil if key doesn't exist
                            }
                        } else {
                            Err("get method can only be called on objects".to_string())
                        }
                    }
                    "has" => {
                        // Object has method: obj.has(key)
                        if let Value::Object(obj) = object_val {
                            let key_val = self.evaluate_expr(argument)?;
                            let key = match key_val {
                                Value::String(s) => s,
                                _ => return Err("has() requires a string key argument".to_string()),
                            };
                            Ok(Value::Boolean(obj.contains_key(&key)))
                        } else {
                            Err("has method can only be called on objects".to_string())
                        }
                    }
                    _ => Err(format!("Unsupported method: {method}")),
                }
            }
            Expr::Transform { from: _, to: _ } => {
                Err("Transform should not be evaluated directly".to_string())
            }
            Expr::FunctionCall { name, arguments } => self.call_function(name, arguments),
            Expr::Nil => Ok(Value::Nil),
        }
    }

    // Call a function with given arguments
    fn call_function(&mut self, name: &str, arguments: &[Expr]) -> Result<Value, String> {
        // Check for built-in functions first
        match name {
            "readline" => self.builtin_readline(arguments),
            "printErr" => self.builtin_print_err(arguments),
            "Date" => self.builtin_date(arguments),
            "Object" => self.builtin_object(arguments),
            _ => {
                // Check for user-defined functions
                let function =
                    if let Some(Value::Function(params, body)) = self.globals.get(name).cloned() {
                        (params, body)
                    } else {
                        return Err(format!("Undefined function '{name}'"));
                    };

                let (params, body) = function;

                // Check argument count
                if arguments.len() != params.len() {
                    return Err(format!(
                        "Function '{name}' expects {} arguments, got {}",
                        params.len(),
                        arguments.len()
                    ));
                }

                // Save current global state
                let saved_globals = self.globals.clone();

                // Evaluate arguments and bind to parameters
                for (param, arg) in params.iter().zip(arguments.iter()) {
                    let arg_value = self.evaluate_expr(arg)?;
                    self.globals.insert(param.clone(), arg_value);
                }

                // Execute function body with return handling
                let result = self.execute_stmt(&body);

                // Restore global state
                self.globals = saved_globals;

                // Handle return value
                match result? {
                    ControlFlow::Return(value) => Ok(value),
                    ControlFlow::None => Ok(Value::Nil),
                }
            }
        }
    }

    // Load a module and import specified names
    fn load_module(&mut self, names: &[String], module_path: &str) -> Result<(), String> {
        use crate::lexer::Lexer;
        use crate::parser::Parser;
        use std::fs;
        use std::path::Path;

        // Ensure the module has .pg extension
        let full_path = if module_path.ends_with(".pg") {
            module_path.to_string()
        } else {
            format!("{module_path}.pg")
        };

        // Try to find the module file
        let module_file = if Path::new(&full_path).exists() {
            full_path.clone()
        } else {
            // Try in examples directory
            let examples_path = format!("examples/{full_path}");
            if Path::new(&examples_path).exists() {
                examples_path
            } else {
                return Err(format!(
                    "Module '{module_path}' not found. Tried: {full_path}, {examples_path}"
                ));
            }
        };

        // Read the module file
        let source = fs::read_to_string(&module_file)
            .map_err(|e| format!("Failed to read module '{module_file}': {e}"))?;

        // Parse the module
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser
            .parse()
            .map_err(|e| format!("Failed to parse module '{module_file}': {e}"))?;

        // Create a temporary interpreter to execute the module
        let mut module_interpreter = Interpreter::new(None);

        // Execute the module to populate its globals
        for stmt in program.statements {
            match module_interpreter.execute_stmt(&stmt)? {
                ControlFlow::Return(_) => {
                    return Err("Return statement not allowed at module level".to_string());
                }
                ControlFlow::None => continue,
            }
        }

        // Import the requested names (only if they start with uppercase)
        for name in names {
            if let Some(value) = module_interpreter.globals.get(name) {
                // Check if the name starts with uppercase (exportable)
                if name
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                {
                    self.globals.insert(name.clone(), value.clone());
                } else {
                    return Err(format!("Cannot import '{name}' - only names starting with uppercase letters can be imported"));
                }
            } else {
                return Err(format!("Name '{name}' not found in module '{module_file}'"));
            }
        }

        Ok(())
    }

    // Built-in function: readline() - Read input from console
    fn builtin_readline(&mut self, arguments: &[Expr]) -> Result<Value, String> {
        if !arguments.is_empty() {
            return Err("readline() takes no arguments".to_string());
        }

        print!("Enter input: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Remove trailing newline
                if input.ends_with('\n') {
                    input.pop();
                    if input.ends_with('\r') {
                        input.pop();
                    }
                }
                Ok(Value::String(input))
            }
            Err(e) => Err(format!("Error reading input: {e}")),
        }
    }

    // Built-in function: printErr() - Print to stderr
    fn builtin_print_err(&mut self, arguments: &[Expr]) -> Result<Value, String> {
        if arguments.len() != 1 {
            return Err("printErr() takes exactly one argument".to_string());
        }

        let value = self.evaluate_expr(&arguments[0])?;
        eprintln!("{value}");
        Ok(Value::Nil)
    }

    // Built-in function: Date() - Create a new Date object
    fn builtin_date(&mut self, arguments: &[Expr]) -> Result<Value, String> {
        match arguments.len() {
            0 => {
                // Current date and time
                Ok(Value::Date(Local::now()))
            }
            1 => {
                // Parse date from string
                let date_str = self.evaluate_expr(&arguments[0])?;
                if let Value::String(s) = date_str {
                    // Try to parse common date formats
                    use chrono::{NaiveDateTime, TimeZone};

                    // Try ISO format first
                    if let Ok(naive) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
                        Ok(Value::Date(
                            Local
                                .from_local_datetime(&naive)
                                .single()
                                .unwrap_or(Local::now()),
                        ))
                    } else if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                    {
                        let naive = naive_date.and_hms_opt(0, 0, 0).unwrap();
                        Ok(Value::Date(
                            Local
                                .from_local_datetime(&naive)
                                .single()
                                .unwrap_or(Local::now()),
                        ))
                    } else {
                        Err(format!("Unable to parse date: '{s}'"))
                    }
                } else {
                    Err("Date() argument must be a string".to_string())
                }
            }
            3 => {
                // Year, month, day
                let year = self.evaluate_expr(&arguments[0])?;
                let month = self.evaluate_expr(&arguments[1])?;
                let day = self.evaluate_expr(&arguments[2])?;

                if let (Value::Number(y), Value::Number(m), Value::Number(d)) = (year, month, day) {
                    use chrono::{NaiveDate, TimeZone};
                    if let Some(naive_date) = NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32)
                    {
                        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
                        Ok(Value::Date(
                            Local
                                .from_local_datetime(&naive_datetime)
                                .single()
                                .unwrap_or(Local::now()),
                        ))
                    } else {
                        Err("Invalid date values".to_string())
                    }
                } else {
                    Err("Date() year, month, and day must be numbers".to_string())
                }
            }
            _ => Err("Date() takes 0, 1, or 3 arguments".to_string()),
        }
    }

    // Built-in function: Object() - Create a new Object
    fn builtin_object(&mut self, _arguments: &[Expr]) -> Result<Value, String> {
        // For now, just create an empty object
        // In a full implementation, we might accept key-value pairs
        Ok(Value::Object(HashMap::new()))
    }
}
