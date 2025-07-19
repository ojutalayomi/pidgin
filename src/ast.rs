// Define the Expr enum, representing all possible expression types in the AST
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64), // Numeric literal expression
    String(String), // String literal expression
    Boolean(bool), // Boolean literal expression
    Identifier(String), // Identifier expression (variable name)
    FixedArray(Vec<Expr>),    // For [a, b, c]
    DynamicArray(Vec<Expr>),  // For {a, b, c}
    Index {
        array: Box<Expr>,     // The array being indexed
        index: Box<Expr>,     // The index expression
    }, // Array indexing: arr[0]
    Nil, // Nil literal expression
    Binary {
        left: Box<Expr>, // Left operand of the binary expression
        operator: BinaryOp, // Operator of the binary expression
        right: Box<Expr>, // Right operand of the binary expression
        line: usize, // Line number of the operator
        column: usize, // Column number of the operator
    },
    Unary {
        operator: UnaryOp, // Operator of the unary expression
        operand: Box<Expr>, // Operand of the unary expression
    },
    Assignment {
        name: String, // Name of the variable being assigned
        value: Box<Expr>, // Value being assigned to the variable
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        argument: Box<Expr>,
    },
    Transform {
        from: String,
        to: String,
    },
    FunctionCall {
        name: String, // Function name
        arguments: Vec<Expr>, // Arguments passed to the function
    },
}

// Define the BinaryOp enum, representing all possible binary operators
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, // Addition operator
    Subtract, // Subtraction operator
    Multiply, // Multiplication operator
    Divide, // Division operator
    Equal, // Equality operator
    NotEqual, // Not-equal operator
    Less, // Less-than operator
    Greater, // Greater-than operator
    LessEqual, // Less-than-or-equal operator
    GreaterEqual, // Greater-than-or-equal operator
}

// Define the UnaryOp enum, representing all possible unary operators
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus, // Unary minus operator (negation)
}

// Define the Stmt enum, representing all possible statement types in the AST
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr), // Expression statement
    Return(Expr), // Return statement
    Print {
        format: Expr,           // The format string expression
        arguments: Vec<Expr>,   // The arguments to be formatted
    }, // Print statement
    Import {
        names: Vec<String>,     // Names to import (can be single or multiple)
        module: String,         // Module file path
    }, // Import statement: GET Alpha from math.pg;
    VarDeclaration {
        name: String, // Name of the variable being declared
        initializer: Option<Expr>, // Optional initializer expression
    },
    FunctionDeclaration {
        name: String, // Name of the function
        parameters: Vec<String>, // Parameter names
        body: Box<Stmt>, // Function body
    },
    Block(Vec<Stmt>), // Block statement (a sequence of statements)
    If {
        condition: Expr, // Condition expression for the if statement
        then_branch: Box<Stmt>, // Statement to execute if condition is true
        else_branch: Option<Box<Stmt>>, // Optional statement to execute if condition is false
    },
    While {
        condition: Expr, // Condition expression for the while loop
        body: Box<Stmt>, // Body of the while loop
    },
}

// Define the Program struct, representing the root of the AST (a list of statements)
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>, // The list of statements in the program
}
