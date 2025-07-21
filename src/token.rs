// Define the Token enum, representing all possible token types in the language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),        // Numeric literal token, stores a floating-point value
    Identifier(String), // Identifier token, stores the variable/function name
    String(String),     // String literal token, stores the string value

    // Operators
    Plus,         // '+' operator token
    Minus,        // '-' operator token
    Star,         // '*' operator token
    Slash,        // '/' operator token
    Assign,       // '=' assignment operator token
    Equal,        // '==' equality operator token
    NotEqual,     // '!=' not-equal operator token
    Less,         // '<' less-than operator token
    Greater,      // '>' greater-than operator token
    LessEqual,    // '<=' less-than-or-equal operator token
    GreaterEqual, // '>=' greater-than-or-equal operator token

    // Keywords
    Let,      // 'let' keyword token
    If,       // 'if' keyword token
    Else,     // 'else' keyword token
    While,    // 'while' keyword token
    Break,    // 'break' keyword
    Print,    // 'print' keyword token
    PrintLn,  // 'printLn' keyword token
    PrintErr, // 'printErr' keyword token
    Function, // 'function' keyword token
    True,     // 'true' boolean literal token
    False,    // 'false' boolean literal token
    Return,   // 'return' keyword token
    Get,      // 'get' keyword token for module imports
    From,     // 'from' keyword token for module imports

    // Delimiters
    LeftParen,    // '(' left parenthesis token
    RightParen,   // ')' right parenthesis token
    LeftBrace,    // '{' left brace token
    RightBrace,   // '}' right brace token
    LeftBracket,  // '[' left bracket token
    RightBracket, // ']' right bracket token
    Semicolon,    // ';' semicolon token
    Comma,        // ',' comma token
    Dot,          // '.' dot token
    Backtick,     // '`' backtick token
    Arrow,        // '->' arrow token
    ArrowLeft,    // '<-' arrow token
    Colon,        // ':' colon token
    ColonEqual,   // ':=' colon-equal token
    AssignRight,  // '=>' arrow-right token
    Imply,        // '<=>' imply token

    // Special
    Newline, // Newline token (for line breaks)
    Eof,     // End-of-file token
}

// Define the TokenInfo struct, which stores a token and its position in the source code
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,  // The token itself
    pub line: usize,   // The line number where the token appears
    pub column: usize, // The column number where the token appears
}

// Implement methods for TokenInfo
impl TokenInfo {
    // Create a new TokenInfo with the given token, line, and column
    pub fn new(token: Token, line: usize, column: usize) -> Self {
        Self {
            token,
            line,
            column,
        } // Return a new TokenInfo instance
    }
}
