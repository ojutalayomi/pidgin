use crate::token::{TokenInfo}; // Import necessary types from the Token module
use crate::ast::{Expr, Stmt, Program, BinaryOp, UnaryOp}; // Import AST types
use std::collections::HashMap; // Import HashMap for variable storage

// Define a custom result type for handling returns
#[derive(Debug, Clone)]
pub enum ControlFlow {
    None,
    Return(Value),
}

// Define the Value enum, representing all possible runtime values
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64), // Numeric value
    String(String), // String value
    Boolean(bool), // Boolean value
    FixedArray(Vec<Value>),
    DynamicArray(Vec<Value>),
    Nil, // Nil (no value)
    Function(Vec<String>, Box<Stmt>), // Function value
}

// Implement methods for Value
impl Value {
    // Check if the value is truthy (for conditionals)
    fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b, // Boolean: use its value
            Value::Nil => false, // Nil is always false
            _ => true, // All other values are truthy
        }
    }
    
    // Convert the value to a string for printing
    fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(), // Convert number to string
            Value::String(s) => s.clone(), // Clone string
            Value::Boolean(b) => b.to_string(), // Convert bool to string
            Value::Nil => "nil".to_string(), // Nil as "nil"
            Value::Function(params, _body) => {
                let params_str = params.join(", ");
                format!("function({}) {{ ... }}", params_str)
            }
            Value::FixedArray(arr) => {
                let elements = arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ");
                format!("[{}]", elements)
            }
            Value::DynamicArray(arr) => {
                let elements = arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ");
                format!("{{{}}}", elements)
            }
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
            tokens: Some(tokens.unwrap_or(Vec::new())),
            current: 0, // Start at the first token
        }
    }
    
    // Interpret a program (execute all statements)
    pub fn interpret(&mut self, program: Program, tokens: Vec<TokenInfo>) -> Result<(), String> {
        self.tokens = Some(tokens);
        self.current = 0; // Reset to the beginning of the token stream
        for statement in program.statements { // Loop through all statements
            match self.execute_stmt(&statement)? { // Execute each statement
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
                    println!("{}", format_value.to_string());
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
                
                    println!("{}", formatted); // Print the value
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
            Stmt::FunctionDeclaration { name, parameters, body } => {
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
                    match self.execute_stmt(stmt)? { // Execute each statement in the block
                        ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                        ControlFlow::None => continue,
                    }
                }
                Ok(ControlFlow::None)
            }
            Stmt::If { condition, then_branch, else_branch } => {
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
                    match self.execute_stmt(body)? { // Execute loop body
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
                    Err(format!("Undefined variable '{}'", name)) // Error if not found
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
                            Err(format!("Array index {} out of bounds (array length: {})", index_num, arr.len()))
                        } else {
                            Ok(arr[index_num].clone())
                        }
                    }
                    _ => Err("Can only index arrays".to_string()),
                }
            }
            Expr::Binary { left, operator, right, line, column } => {
                let left_val = &self.evaluate_expr(left)?; // Evaluate left operand
                let right_val = &self.evaluate_expr(right)?; // Evaluate right operand
                
                match operator {
                    BinaryOp::Add => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)), // Add numbers
                        (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))), // Concatenate strings
                        (Value::String(a), Value::Number(b)) => Ok(Value::String(format!("{}{}", a, b))), // String + number
                        (Value::Number(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))), // Number + string
                        (Value::String(a), Value::Boolean(b)) => Ok(Value::String(format!("{}{}", a, b))), // String + bool
                        (Value::Boolean(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))), // Bool + string
                        _ => {
                            Err(format!("Invalid operands for addition: {:?} + {:?} at line {} column {}", left_val, right_val, line, column))
                        }
                    },
                    BinaryOp::Subtract => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)), // Subtract numbers
                        _ => {
                            Err(format!("Invalid operands for subtraction: {:?} - {:?} at line {} column {}", left_val, right_val, line, column))
                        }
                    },
                    BinaryOp::Multiply => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)), // Multiply numbers
                        _ => {
                            Err(format!("Invalid operands for multiplication: {:?} * {:?} at line {} column {}", left_val, right_val, line, column))
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
                            Err(format!("Invalid operands for division: {:?} / {:?} at line {} column {}", left_val, right_val, line, column))
                        }
                    },
                    BinaryOp::Equal => Ok(Value::Boolean(self.is_equal(&left_val, &right_val))), // Equality check
                    BinaryOp::NotEqual => Ok(Value::Boolean(!self.is_equal(&left_val, &right_val))), // Not-equal check
                    BinaryOp::Greater => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)), // Greater than
                        _ => {
                            Err(format!("Invalid operands for comparison: {:?} > {:?} at line {} column {}", left_val, right_val, line, column))
                        }
                    },
                    BinaryOp::GreaterEqual => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)), // Greater or equal
                        _ => {
                            Err(format!("Invalid operands for comparison: {:?} >= {:?} at line {} column {}", left_val, right_val, line, column))
                        }
                    },
                    BinaryOp::Less => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)), // Less than
                        _ => {
                            Err(format!("Invalid operands for comparison: {:?} < {:?} at line {} column {}", left_val, right_val, line, column))
                        }
                    },
                    BinaryOp::LessEqual => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)), // Less or equal
                        _ => {
                            Err(format!("Invalid operands for comparison: {:?} <= {:?} at line {} column {}", left_val, right_val, line, column))
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
            Expr::MethodCall { object, method, argument } => {
                if method == "replaceChar" {
                    let object_val = self.evaluate_expr(object)?;
                    if let Value::String(original) = object_val {
                        if let Expr::Transform { from, to } = argument.as_ref() {
                            // Try to resolve 'from' as a variable, fallback to literal if not found
                            let from_value = if let Some(val) = self.globals.get(from) {
                                match val {
                                    Value::String(s) => s.clone(),
                                    Value::Number(n) => n.to_string(),
                                    Value::Boolean(b) => b.to_string(),
                                    _ => return Err(format!("Variable '{}' is not a valid replacement value", from)),
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
                                    _ => return Err(format!("Variable '{}' is not a valid replacement value", to)),
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
                } else if method == "push" {
                    // Array push method: arr.push(value)
                    let object_val = self.evaluate_expr(object)?;
                    let arg_val = self.evaluate_expr(argument)?;
                    
                    match object_val {
                        Value::DynamicArray(mut arr) => {
                            arr.push(arg_val);
                            Ok(Value::DynamicArray(arr))
                        }
                        _ => Err("Push method can only be called on dynamic arrays".to_string()),
                    }
                } else if method == "pop" {
                    // Array pop method: arr.pop()
                    let object_val = self.evaluate_expr(object)?;
                    
                    // Verify no argument was provided
                    if let Expr::Nil = argument.as_ref() {
                        match object_val {
                            Value::DynamicArray(mut arr) => {
                                if arr.is_empty() {
                                    Err("Cannot pop from empty array".to_string())
                                } else {
                                    let popped = arr.pop().unwrap();
                                    Ok(popped)
                                }
                            }
                            _ => Err("Pop method can only be called on dynamic arrays".to_string()),
                        }
                    } else {
                        Err("Pop method does not take arguments".to_string())
                    }
                } else if method == "length" {
                    // Array length method: arr.length()
                    let object_val = self.evaluate_expr(object)?;
                    
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
                } else if method == "clear" {
                    // Array clear method: arr.clear()
                    let object_val = self.evaluate_expr(object)?;
                    
                    // Verify no argument was provided
                    if let Expr::Nil = argument.as_ref() {
                        match object_val {
                            Value::DynamicArray(_) => {
                                Ok(Value::DynamicArray(Vec::new()))
                            }
                            _ => Err("Clear method can only be called on dynamic arrays".to_string()),
                        }
                    } else {
                        Err("Clear method does not take arguments".to_string())
                    }
                } else {
                    Err(format!("Unsupported method: {}", method))
                }
            }
            Expr::Transform { from: _, to: _ } => {
                Err("Transform should not be evaluated directly".to_string())
            }
            Expr::FunctionCall { name, arguments } => {
                self.call_function(name, arguments)
            }
            Expr::Nil => {
                Ok(Value::Nil)
            }
        }
    }
    

    // Call a function with given arguments
    fn call_function(&mut self, name: &str, arguments: &[Expr]) -> Result<Value, String> {
        // Get the function from globals
        let function = if let Some(Value::Function(params, body)) = self.globals.get(name).cloned() {
            (params, body)
        } else {
            return Err(format!("Undefined function '{}'", name));
        };
        
        let (params, body) = function;
        
        // Check argument count
        if arguments.len() != params.len() {
            return Err(format!(
                "Function '{}' expects {} arguments, got {}",
                name,
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
    
    // Check if two values are equal
    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => a == b, // Compare numbers
            (Value::String(a), Value::String(b)) => a == b, // Compare strings
            (Value::Boolean(a), Value::Boolean(b)) => a == b, // Compare booleans
            (Value::Nil, Value::Nil) => true, // Both nil
            _ => false, // Otherwise, not equal
        }
    }
    
    // Load a module and import specified names
    fn load_module(&mut self, names: &[String], module_path: &str) -> Result<(), String> {
        use std::fs;
        use std::path::Path;
        use crate::lexer::Lexer;
        use crate::parser::Parser;
        
        // Ensure the module has .pg extension
        let full_path = if module_path.ends_with(".pg") {
            module_path.to_string()
        } else {
            format!("{}.pg", module_path)
        };
        
        // Try to find the module file
        let module_file = if Path::new(&full_path).exists() {
            full_path.clone()
        } else {
            // Try in examples directory
            let examples_path = format!("examples/{}", full_path);
            if Path::new(&examples_path).exists() {
                examples_path
            } else {
                return Err(format!("Module '{}' not found. Tried: {}, {}", module_path, full_path, examples_path));
            }
        };
        
        // Read the module file
        let source = fs::read_to_string(&module_file)
            .map_err(|e| format!("Failed to read module '{}': {}", module_file, e))?;
        
        // Parse the module
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse()
            .map_err(|e| format!("Failed to parse module '{}': {}", module_file, e))?;
        
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
}
