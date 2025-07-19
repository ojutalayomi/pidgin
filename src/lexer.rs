// Import the Token and TokenInfo types from the token module
use crate::token::{Token, TokenInfo};

// Define the Lexer struct, which will be responsible for tokenizing input source code
pub struct Lexer {
    input: Vec<char>, // The input source code as a vector of characters
    position: usize,  // The current position in the input
    line: usize,      // The current line number (for error reporting)
    column: usize,    // The current column number (for error reporting)
}

// Implement methods for the Lexer struct
impl Lexer {
    // Create a new Lexer from a string slice
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(), // Convert the input string to a vector of chars
            position: 0,                    // Start at the beginning of the input
            line: 1,                        // Start at line 1
            column: 1,                      // Start at column 1
        }
    }

    // Tokenize the input and return a vector of TokenInfo
    pub fn tokenize(&mut self) -> Vec<TokenInfo> {
        let mut tokens = Vec::new(); // Create a vector to store tokens

        while !self.is_at_end() {
            // Loop until the end of input
            self.skip_whitespace(); // Skip whitespace characters

            if self.is_at_end() {
                // Check again in case we reached the end
                break; // Exit the loop
            }

            let line = self.line; // Store the current line for the token
            let column = self.column; // Store the current column for the token

            match self.current_char() {
                // Match on the current character
                '`' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::Backtick, line, column)); // Add a Backtick token
                }
                '+' => {
                    self.advance(); // Move to the next character
                    tokens.push(TokenInfo::new(Token::Plus, line, column)); // Add a Plus token
                }
                '-' => {
                    self.advance();
                    if self.current_char() == '>' {
                        self.advance();
                        tokens.push(TokenInfo::new(Token::Arrow, line, column));
                    // Add an Arrow token
                    } else {
                        tokens.push(TokenInfo::new(Token::Minus, line, column));
                        // Add a Minus token
                    }
                }
                '*' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::Star, line, column)); // Add a Star token
                }
                '.' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::Dot, line, column)); // Add a Dot token
                }
                '/' => {
                    self.advance();
                    if self.current_char() == '/' {
                        // Check for comment
                        // Skip comment until end of line
                        while !self.is_at_end() && self.current_char() != '\n' {
                            self.advance(); // Skip each character in the comment
                        }
                    } else {
                        tokens.push(TokenInfo::new(Token::Slash, line, column));
                        // Add a Slash token
                    }
                }
                '=' => {
                    self.advance();
                    if self.current_char() == '=' {
                        self.advance();
                        tokens.push(TokenInfo::new(Token::Equal, line, column));
                    // Add an Equal token (==)
                    } else {
                        tokens.push(TokenInfo::new(Token::Assign, line, column));
                        // Add an Assign token (=)
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char() == '=' {
                        self.advance();
                        tokens.push(TokenInfo::new(Token::NotEqual, line, column));
                    // Add a NotEqual token (!=)
                    } else {
                        panic!("Unexpected character '!' at line {line}, column {column}");
                        // Error for lone '!'
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char() == '=' {
                        self.advance();
                        tokens.push(TokenInfo::new(Token::LessEqual, line, column));
                    // Add a LessEqual token (<=)
                    } else if self.current_char() == '-' {
                        self.advance();
                        tokens.push(TokenInfo::new(Token::ArrowLeft, line, column));
                    // Add a ArrowLeft token (<-)
                    } else {
                        tokens.push(TokenInfo::new(Token::Less, line, column)); // Add a Less token (<)
                    }
                }
                '>' => {
                    self.advance();
                    if self.current_char() == '=' {
                        self.advance();
                        tokens.push(TokenInfo::new(Token::GreaterEqual, line, column));
                    // Add a GreaterEqual token (>=)
                    } else {
                        tokens.push(TokenInfo::new(Token::Greater, line, column));
                        // Add a Greater token (>)
                    }
                }
                '(' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::LeftParen, line, column));
                    // Add a LeftParen token
                }
                ')' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::RightParen, line, column));
                    // Add a RightParen token
                }
                '{' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::LeftBrace, line, column));
                    // Add a LeftBrace token
                }
                '}' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::RightBrace, line, column));
                    // Add a RightBrace token
                }
                '[' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::LeftBracket, line, column));
                    // Add a LeftBracket token
                }
                ']' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::RightBracket, line, column));
                    // Add a RightBracket token
                }
                ';' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::Semicolon, line, column));
                    // Add a Semicolon token
                }
                ',' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::Comma, line, column)); // Add a Comma token
                }
                '\n' => {
                    self.advance();
                    tokens.push(TokenInfo::new(Token::Newline, line, column)); // Add a Newline token
                }
                '"' => {
                    let string_literal = self.scan_string(); // Parse a string literal
                    tokens.push(TokenInfo::new(Token::String(string_literal), line, column));
                    // Add a String token
                }
                c if c.is_ascii_digit() => {
                    let number = self.scan_number(); // Parse a number literal
                    tokens.push(TokenInfo::new(Token::Number(number), line, column));
                    // Add a Number token
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let identifier = self.scan_identifier(); // Parse an identifier or keyword
                    let token = self.keyword_or_identifier(identifier); // Determine if it's a keyword or identifier
                    tokens.push(TokenInfo::new(token, line, column)); // Add the token
                }
                _ => {
                    panic!(
                        "Unexpected character '{}' at line {}, column {}",
                        self.current_char(),
                        line,
                        column
                    ); // Error for unknown character
                }
            }
        }

        tokens.push(TokenInfo::new(Token::Eof, self.line, self.column)); // Add an EOF token at the end
        tokens // Return the vector of tokens
    }

    // Get the current character, or '\0' if at the end
    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0' // Null character if at end
        } else {
            self.input[self.position] // Current character
        }
    }

    // Advance to the next character and return the current one
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0' // Null character if at end
        } else {
            let ch = self.input[self.position]; // Get current character
            self.position += 1; // Move position forward

            if ch == '\n' {
                self.line += 1; // Increment line number on newline
                self.column = 1; // Reset column to 1
            } else {
                self.column += 1; // Increment column otherwise
            }

            ch // Return the character
        }
    }

    // Check if we've reached the end of the input
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len() // True if position is past input length
    }

    // Skip whitespace characters (space, carriage return, tab)
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.current_char() {
                ' ' | '\r' | '\t' => {
                    self.advance(); // Skip whitespace
                }
                _ => break, // Stop on non-whitespace
            }
        }
    }

    // Scan and return a string literal (handles escape sequences)
    fn scan_string(&mut self) -> String {
        self.advance(); // Skip opening quote
        let mut value = String::new(); // Store the string value

        while !self.is_at_end() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance(); // Skip the backslash
                match self.current_char() {
                    'n' => value.push('\n'),  // Newline escape
                    't' => value.push('\t'),  // Tab escape
                    'r' => value.push('\r'),  // Carriage return escape
                    '\\' => value.push('\\'), // Backslash escape
                    '"' => value.push('"'),   // Quote escape
                    _ => {
                        value.push('\\'); // Unknown escape, keep backslash
                        value.push(self.current_char()); // Add the character
                    }
                }
            } else {
                value.push(self.current_char()); // Add normal character
            }
            self.advance(); // Move to next character
        }

        if self.is_at_end() {
            panic!(
                "Unterminated string at line {} column {}",
                self.line, self.column
            ); // Error if string not closed
        }

        self.advance(); // Skip closing quote
        value // Return the string value
    }

    // Scan and return a number literal as f64
    fn scan_number(&mut self) -> f64 {
        let mut value = String::new(); // Store the number as a string

        while !self.is_at_end()
            && (self.current_char().is_ascii_digit() || self.current_char() == '.')
        {
            value.push(self.current_char()); // Add digit or dot
            self.advance(); // Move to next character
        }

        value.parse().unwrap_or_else(|_| {
            panic!(
                "Invalid number '{}' at line {}, column {}",
                value, self.line, self.column
            ); // Error if not a valid number
        })
    }

    // Scan and return an identifier (or keyword) as a String
    fn scan_identifier(&mut self) -> String {
        let mut value = String::new(); // Store the identifier

        while !self.is_at_end()
            && (self.current_char().is_ascii_alphanumeric() || self.current_char() == '_')
        {
            value.push(self.current_char()); // Add character
            self.advance(); // Move to next character
        }

        value // Return the identifier
    }

    // Determine if a string is a keyword or an identifier
    fn keyword_or_identifier(&self, text: String) -> Token {
        match text.to_lowercase().as_str() {
            "let" => Token::Let,           // let keyword
            "if" => Token::If,             // if keyword
            "else" => Token::Else,         // else keyword
            "while" => Token::While,       // while keyword
            "break" => Token::Break,       // break keyword
            "print" => Token::Print,       // print keyword
            "function" => Token::Function, // function keyword
            "true" => Token::True,         // true keyword
            "false" => Token::False,       // false keyword
            "return" => Token::Return,     // return keyword
            "get" => Token::Get,           // get keyword for imports
            "from" => Token::From,         // from keyword for imports
            "<-" => Token::ArrowLeft,      // <- arrow token for imports
            _ => Token::Identifier(text),  // Otherwise, it's an identifier
        }
    }
}
