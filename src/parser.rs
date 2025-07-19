// Import necessary modules and types
use crate::token::{Token, TokenInfo}; // Import Token and TokenInfo from token.rs
use crate::ast::{Expr, Stmt, Program, BinaryOp, UnaryOp}; // Import AST types

// Define the Parser struct, which will parse tokens into an AST
pub struct Parser {
    tokens: Vec<TokenInfo>, // The list of tokens to parse
    current: usize,         // The current position in the token list
}

// Implement methods for the Parser struct
impl Parser {
    // Create a new Parser from a vector of tokens
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            tokens,      // Store the tokens
            current: 0,  // Start at the first token
        }
    }

    // Parse the tokens into a Program (AST root)
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new(); // Store parsed statements
        while !self.is_at_end() { // Loop until all tokens are parsed
            // Skip newlines at the top level
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.statement()?); // Parse a statement and add to the list
        }
        Ok(Program { statements }) // Return the program AST
    }

    // Parse a statement
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&Token::Get) { // Check for import statement
            return self.import_statement(); // Parse import statement
        }
        if self.match_token(&Token::Return) { // Check for return statement
            return self.return_statement(); // Parse return statement
        }
        if self.match_token(&Token::Print) { // Check for print statement
            return self.print_statement(); // Parse print statement
        }
        if self.match_token(&Token::Let) { // Check for variable declaration
            return self.var_declaration(); // Parse variable declaration
        }
        if self.match_token(&Token::Function) { // Check for function declaration
            return self.function_declaration(); // Parse function declaration
        }
        if self.match_token(&Token::If) { // Check for if statement
            return self.if_statement(); // Parse if statement
        }
        if self.match_token(&Token::While) { // Check for while statement
            return self.while_statement(); // Parse while statement
        }
        if self.match_token(&Token::LeftBrace) { // Check for block statement
            return self.block_statement(); // Parse block statement
        }
        self.expression_statement() // Otherwise, parse as expression statement
    }

    // Parse a print statement
    fn print_statement(&mut self) -> Result<Stmt, String> {
        let (format_expr, arguments) = if self.match_token(&Token::LeftParen) {
            // Parenthesized form: print("{}", name3);
            let format_expr = self.expression()?;
            let mut arguments = Vec::new();
            while self.match_token(&Token::Comma) {
                arguments.push(self.expression()?);
            }
            self.consume(&Token::RightParen, "Expect ')' after print arguments.")?;
            (format_expr, arguments)
        } else {
            // Non-parenthesized form: print "{}", name3;
            let format_expr = self.expression()?;
            let mut arguments = Vec::new();
            while self.match_token(&Token::Comma) {
                arguments.push(self.expression()?);
            }
            (format_expr, arguments)
        };
    
        self.consume(&Token::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { format: format_expr, arguments })
    }

    // Parse a return statement
    fn return_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?; // Parse the expression to return
        self.consume(&Token::Semicolon, "Expect ';' after return value.")?; // Expect a semicolon
        Ok(Stmt::Return(expr)) // Return a Return statement
    }

    // Parse an import statement
    fn import_statement(&mut self) -> Result<Stmt, String> {
        let mut names = Vec::new();
        
        // Parse the names to import
        if self.match_token(&Token::LeftBrace) {
            // Multiple names: GET {Alpha,B} from math.pg;
            loop {
                let name_token = self.consume_identifier("Expect identifier in import list")?;
                if let Token::Identifier(name) = &name_token.token {
                    names.push(name.clone());
                } else {
                    return Err("Invalid identifier in import list".to_string());
                }
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
            self.consume(&Token::RightBrace, "Expect '}' after import list")?;
        } else {
            // Single name: GET Alpha from math.pg;
            let name_token = self.consume_identifier("Expect identifier after GET")?;
            if let Token::Identifier(name) = &name_token.token {
                names.push(name.clone());
            } else {
                return Err("Invalid identifier after GET".to_string());
            }
        }
        
        // Parse "from" or "<-" keyword
        let from_token = self.peek();
        match &from_token.token {
            Token::From => {
                self.advance();
            },
            Token::ArrowLeft => {
                self.advance(); 
            },
            _ => {
                return Err(format!("Expect 'from' or '<-' after import names, got {from_token:?} at line {line} column {column}", from_token = from_token.token, line = from_token.line, column = from_token.column));
            }
        }
        
        // Parse module path (handle dot notation)
        let mut module_parts = Vec::new();
        
        // First part of the path
        let first_part = self.consume_identifier("Expect module path")?;
        if let Token::Identifier(part) = &first_part.token {
            module_parts.push(part.clone());
        } else {
            return Err("Invalid module path".to_string());
        }
        
        // Handle additional parts with dots (e.g., math.pg)
        while self.match_token(&Token::Dot) {
            let part_token = self.consume_identifier("Expect identifier after dot in module path")?;
            if let Token::Identifier(part) = &part_token.token {
                module_parts.push(part.clone());
            } else {
                return Err("Invalid identifier after dot in module path".to_string());
            }
        }
        
        let module = module_parts.join(".");
        
        self.consume(&Token::Semicolon, "Expect ';' after import statement")?;
        
        Ok(Stmt::Import { names, module })
    }

    // Parse a variable declaration
    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.consume_identifier("Expect variable name.")?; // Expect an identifier
        let name = if let Token::Identifier(n) = &name_token.token {
            n.clone() // Get the variable name
        } else {
            return Err("Invalid variable name.".to_string()); // Error if not an identifier
        };
        let initializer = if self.match_token(&Token::Assign) { // Check for initializer
            Some(self.expression()?) // Parse the initializer expression
        } else {
            None // No initializer
        };
        self.consume(&Token::Semicolon, "Expect ';' after variable declaration.")?; // Expect a semicolon
        Ok(Stmt::VarDeclaration { name, initializer }) // Return a VarDeclaration statement
    }

    // Parse a function declaration
    fn function_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.consume_identifier("Expect function name.")?; // Expect function name
        let name = if let Token::Identifier(n) = &name_token.token {
            n.clone() // Get the function name
        } else {
            return Err("Invalid function name.".to_string()); // Error if not an identifier
        };
        
        self.consume(&Token::LeftParen, "Expect '(' after function name.")?; // Expect '('
        
        let mut parameters = Vec::new(); // Store parameter names
        
        // Parse parameters
        if !self.check(&Token::RightParen) {
            loop {
                let param_token = self.consume_identifier("Expect parameter name.")?; // Expect parameter
                if let Token::Identifier(param_name) = &param_token.token {
                    parameters.push(param_name.clone()); // Add parameter to list
                } else {
                    return Err("Invalid parameter name.".to_string()); // Error if not identifier
                }
                
                if !self.match_token(&Token::Comma) { // Check for comma
                    break; // No comma, end of parameters
                }
            }
        }
        
        self.consume(&Token::RightParen, "Expect ')' after parameters.")?; // Expect ')'
        
        self.consume(&Token::LeftBrace, "Expect '{' before function body.")?; // Expect '{'
        
        let mut body_statements = Vec::new(); // Store statements in the function body
        while !self.check(&Token::RightBrace) && !self.is_at_end() { // Loop until '}' or end
            // Skip newlines in function body
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            body_statements.push(self.statement()?); // Parse and add each statement
        }
        self.consume(&Token::RightBrace, "Expect '}' after function body.")?; // Expect '}'
        
        let body = Box::new(Stmt::Block(body_statements)); // Create block statement
        
        Ok(Stmt::FunctionDeclaration { name, parameters, body }) // Return function declaration
    }

    // Parse an if statement
    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(&Token::LeftParen, "Expect '(' after 'if'.")?; // Expect '('
        let condition = self.expression()?; // Parse the condition expression
        self.consume(&Token::RightParen, "Expect ')' after if condition.")?; // Expect ')'
        let then_branch = Box::new(self.statement()?); // Parse the then branch
        let else_branch = if self.match_token(&Token::Else) { // Check for else branch
            Some(Box::new(self.statement()?)) // Parse the else branch
        } else {
            None // No else branch
        };
        Ok(Stmt::If { condition, then_branch, else_branch }) // Return an If statement
    }

    // Parse a while statement
    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(&Token::LeftParen, "Expect '(' after 'while'.")?; // Expect '('
        let condition = self.expression()?; // Parse the condition expression
        self.consume(&Token::RightParen, "Expect ')' after condition.")?; // Expect ')'
        let body = Box::new(self.statement()?); // Parse the loop body
        Ok(Stmt::While { condition, body }) // Return a While statement
    }

    // Parse a block statement (a sequence of statements in braces)
    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = Vec::new(); // Store statements in the block
        while !self.check(&Token::RightBrace) && !self.is_at_end() { // Loop until '}' or end
            // Skip newlines in block
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.statement()?); // Parse and add each statement
        }
        self.consume(&Token::RightBrace, "Expect '}' after block.")?; // Expect '}'
        Ok(Stmt::Block(statements)) // Return a Block statement
    }

    // Parse an expression statement
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?; // Parse the expression
        self.consume(&Token::Semicolon, "Expect ';' after expression.")?; // Expect a semicolon
        Ok(Stmt::Expression(expr)) // Return an Expression statement
    }

    // Parse an expression
    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment() // Start with assignment expression
    }

    // Parse an assignment expression
    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.equality()?; // Parse equality expression
        if self.match_token(&Token::Assign) { // Check for assignment
            let _equals = self.previous(); // Get the '=' token
            let value = self.assignment()?; // Parse the right-hand side
            if let Expr::Identifier(name) = expr {
                return Ok(Expr::Assignment { name, value: Box::new(value) }); // Return Assignment expression
            }
            return Err("Invalid assignment target.".to_string()); // Error if not an identifier
        }
        Ok(expr) // Return the parsed expression
    }

    // Parse an equality expression (==, !=)
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?; // Parse comparison expression
        while self.match_token(&Token::Equal) || self.match_token(&Token::NotEqual) { // Loop for == or !=
            let previous_token = self.previous();
            let operator = match previous_token.token {
                Token::Equal => BinaryOp::Equal, // Map to BinaryOp::Equal
                Token::NotEqual => BinaryOp::NotEqual, // Map to BinaryOp::NotEqual
                _ => unreachable!(), // Should not happen
            };
            let right = self.comparison()?; // Parse right operand
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                line: previous_token.line,
                column: previous_token.column,
            };
        }
        Ok(expr) // Return the parsed expression
    }

    // Parse a comparison expression (<, >, <=, >=)
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?; // Parse term expression
        while self.match_token(&Token::Less)
            || self.match_token(&Token::LessEqual)
            || self.match_token(&Token::Greater)
            || self.match_token(&Token::GreaterEqual)
        {
            let previous_token = self.previous();
            let operator = match previous_token.token {
                Token::Less => BinaryOp::Less, // Map to BinaryOp::Less
                Token::LessEqual => BinaryOp::LessEqual, // Map to BinaryOp::LessEqual
                Token::Greater => BinaryOp::Greater, // Map to BinaryOp::Greater
                Token::GreaterEqual => BinaryOp::GreaterEqual, // Map to BinaryOp::GreaterEqual
                _ => unreachable!(), // Should not happen
            };
            let right = self.term()?; // Parse right operand
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                line: previous_token.line,
                column: previous_token.column,
            };
        }
        Ok(expr) // Return the parsed expression
    }

    // Parse a term expression (+, -)
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?; // Parse factor expression
        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let previous_token = self.previous();
            let operator = match previous_token.token {
                Token::Plus => BinaryOp::Add, // Map to BinaryOp::Add
                Token::Minus => BinaryOp::Subtract, // Map to BinaryOp::Subtract
                _ => unreachable!(), // Should not happen
            };
            let right = self.factor()?; // Parse right operand
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                line: previous_token.line,
                column: previous_token.column,
            };
        }
        Ok(expr) // Return the parsed expression
    }

    // Parse a factor expression (*, /)
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?; // Parse unary expression
        while self.match_token(&Token::Star) || self.match_token(&Token::Slash) {
            let previous_token = self.previous();
            let operator = match previous_token.token {
                Token::Star => BinaryOp::Multiply, // Map to BinaryOp::Multiply
                Token::Slash => BinaryOp::Divide, // Map to BinaryOp::Divide
                _ => unreachable!(), // Should not happen
            };
            let right = self.unary()?; // Parse right operand
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                line: previous_token.line,
                column: previous_token.column,
            };
        }
        Ok(expr) // Return the parsed expression
    }

    // Parse a unary expression (-)
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&Token::Minus) {
            let operator = UnaryOp::Minus; // Only minus is supported
            let operand = self.unary()?; // Parse the operand
            return Ok(Expr::Unary {
                operator,
                operand: Box::new(operand),
            });
        }
        self.primary() // Otherwise, parse as primary expression
    }

    // Parse a primary expression (literals, identifiers, parenthesized expressions)
    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.advance(); // Get the next token
        let mut expr = match &token.token {
            Token::Number(n) => Ok(Expr::Number(*n)), // Numeric literal
            Token::String(s) => Ok(Expr::String(s.clone())), // String literal
            Token::True => Ok(Expr::Boolean(true)), // true literal
            Token::False => Ok(Expr::Boolean(false)), // false literal
            Token::Identifier(name) => Ok(Expr::Identifier(name.clone())), // Identifier
            Token::LeftParen => {
                let expr = self.expression()?; // Parse the inner expression
                self.consume(&Token::RightParen, "Expect ')' after expression.")?; // Expect ')'
                Ok(expr) // Return the inner expression
            },
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
                let mut elements = Vec::new();
                if !self.check(&Token::RightBrace) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&Token::RightBrace, "Expect '}' after array elements.")?;
                Ok(Expr::DynamicArray(elements))
            }
                _ => Err("Expect expression.".to_string()), // Error for invalid primary
            }?;
        
        // Check for function calls
        while self.check(&Token::LeftParen) {
            self.advance(); // consume '('
            
            // Parse arguments
            let mut arguments = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    arguments.push(self.expression()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            
            self.consume(&Token::RightParen, "Expect ')' after arguments.")?;
            
            // Convert identifier to function call
            if let Expr::Identifier(name) = expr {
                expr = Expr::FunctionCall { name, arguments };
            } else {
                return Err("Only identifiers can be called as functions.".to_string());
            }
        }
        
        // Check for method calls
        while self.check(&Token::Dot) {
            self.advance(); // consume '.'
            let method_name = match &self.peek().token {
                Token::Identifier(name) => name.clone(),
                Token::Get => "get".to_string(), // Handle 'get' as method name
                _ => return Err("Expect method name after '.'.".to_string()),
            };
            self.advance(); // consume method name
            
            // Parse method arguments based on method type
            let argument = if method_name == "replaceChar" {
                // Special case for replaceChar with backtick syntax
                self.consume(&Token::Backtick, "Expect '`' after 'replaceChar'")?;
                let from = self.parse_transform("from")?;
                
                self.consume(&Token::Arrow, "Expect '->' in transform")?;

                let to = self.parse_transform("to")?;
                
                self.consume(&Token::Backtick, "Expect '`' to close transform")?;
                
                Expr::Transform { from, to }
            } else if method_name == "push" {
                // push method requires an argument
                self.consume(&Token::LeftParen, "Expect '(' after 'push'")?;
                let arg = self.expression()?;
                self.consume(&Token::RightParen, "Expect ')' after push argument")?;
                arg
            } else if method_name == "pop" || method_name == "length" || method_name == "clear" || 
                      method_name == "reverse" || method_name == "toUpper" || method_name == "toLower" || 
                      method_name == "trim" || method_name == "getYear" || method_name == "getMonth" || 
                      method_name == "getDay" || method_name == "keys" {
                // These methods don't take arguments
                self.consume(&Token::LeftParen, "Expect '(' after method name")?;
                self.consume(&Token::RightParen, "Expect ')' after method name")?;
                Expr::Nil // Use Nil as placeholder for no argument
            } else if method_name == "insert" || method_name == "set" {
                // These methods take two arguments: (arg1, arg2)
                self.consume(&Token::LeftParen, "Expect '(' after method name")?;
                let arg1 = self.expression()?;
                self.consume(&Token::Comma, "Expect ',' between arguments")?;
                let arg2 = self.expression()?;
                self.consume(&Token::RightParen, "Expect ')' after arguments")?;
                Expr::Binary {
                    left: Box::new(arg1),
                    operator: crate::ast::BinaryOp::Add, // Use Add as placeholder, will be ignored
                    right: Box::new(arg2),
                    line: 0,
                    column: 0,
                }
            } else if method_name == "remove" || method_name == "get" || method_name == "has" {
                // These methods take one argument
                self.consume(&Token::LeftParen, "Expect '(' after method name")?;
                let arg = self.expression()?;
                self.consume(&Token::RightParen, "Expect ')' after argument")?;
                arg
            } else if method_name == "format" {
                // format method takes one argument
                self.consume(&Token::LeftParen, "Expect '(' after method name")?;
                let arg = self.expression()?;
                self.consume(&Token::RightParen, "Expect ')' after argument")?;
                arg
            } else {
                return Err(format!("Unsupported method: {method_name}"));
            };
            
            expr = Expr::MethodCall {
                object: Box::new(expr),
                method: method_name,
                argument: Box::new(argument),
            };
        }
        
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
        
        Ok(expr)
    }

    fn parse_transform(&mut self, transform_type: &str) -> Result<String, String> {
        if self.match_token(&Token::LeftBrace) {
            // Handle {var} syntax
            if let Token::Identifier(name) = &self.peek().token {
                let var = name.clone();
                self.advance();
                self.consume(&Token::RightBrace, "Expect '}' after variable name")?;
                Ok(var)
            } else {
                Err("Expect variable name in transform".to_string())
            }
        } else {
            // Continue without requiring {var}
            if let Token::Identifier(name) = &self.peek().token {
                let var = name.clone();
                self.advance();
                Ok(var)
            } else {
                Err(format!("Expect '{transform_type}' pattern in transform"))
            }
        }
    }

    // Check if the current token matches the given token type
    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            false // No more tokens
        } else {
            &self.peek().token == token_type // Compare token types
        }
    }

    // Advance and return the current token if it matches the given type
    fn match_token(&mut self, token_type: &Token) -> bool {
        if self.check(token_type) {
            self.advance(); // Move to next token
            true // Matched
        } else {
            false // Not matched
        }
    }

    // Advance and return the current token
    fn advance(&mut self) -> TokenInfo {
        if !self.is_at_end() {
            self.current += 1; // Move to next token
        }
        self.tokens[self.current - 1].clone() // Return the previous token
    }

    // Check if we've reached the end of the token list
    fn is_at_end(&self) -> bool {
        self.peek().token == Token::Eof // True if current token is EOF
    }

    // Peek at the current token without advancing
    fn peek(&self) -> &TokenInfo {
        &self.tokens[self.current] // Return current token
    }

    // Get the previous token
    fn previous(&self) -> TokenInfo {
        self.tokens[self.current - 1].clone() // Return previous token
    }

    // Consume the current token if it matches the expected type, or return an error
    fn consume(&mut self, token_type: &Token, message: &str) -> Result<TokenInfo, String> {
        if self.check(token_type) {
            Ok(self.advance()) // Return the token
        } else {
            Err(format!("{message} at line {line} column {column}", line = self.peek().line, column = self.peek().column)) // Error if not matched
        }
    }

    // Consume and return an identifier token, or return an error
    fn consume_identifier(&mut self, message: &str) -> Result<TokenInfo, String> {
        let token = self.advance(); // Get the next token
        match &token.token {
            Token::Identifier(_) => Ok(token), // Return if it's an identifier
            _ => Err(format!("{message} at line {line} column {column}", line = token.line, column = token.column)), // Error otherwise
        }
    }
}