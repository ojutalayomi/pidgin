// Import the token module
mod token; // Handles token definitions and tokenization
// Import the lexer module
mod lexer; // Handles lexical analysis (tokenizing source code)
// Import the ast module
mod ast; // Defines the abstract syntax tree (AST) structures
// Import the parser module
mod parser; // Handles parsing tokens into AST
// Import the interpreter module
mod interpreter; // Handles interpreting/executing the AST

use std::env; // Import for reading command-line arguments
use std::fs; // Import for file system operations
use std::io::{self, Write}; // Import for input/output
use crate::interpreter::Interpreter; // Import the Interpreter struct

// The main entry point of the program
fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments
    if args.len() > 1 {
        let first_arg = &args[1]; // Get the first argument
        
        // Check for standalone flags first
        match first_arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--version" | "-v" => {
                display_version();
                return;
            }
            _ => {}
        }
        
        // If not a standalone flag, treat as file path
        let path = first_arg;
        
        // Check file extension (for all file operations)
        if !path.ends_with(".pg") {
            let dot_index = path.rfind('.').unwrap_or(path.len());
            let ext = &path[dot_index..];
            panic!("Expected .pg file but got {} from {}", ext, path);
        }

        // Check for file-specific flags
        if args.len() > 2 {
            match args[2].as_str() {
                "--tokens" => {
                    display_tokens(path);
                    return;
                }
                "--ast" => {
                    display_ast(path);
                    return;
                }
                "--help" => {
                    print_help();
                    return;
                }
                "--version" => {
                    display_version();
                    return;
                }
                                    _ => {
                        eprintln!("Unknown flag: {}", args[2]);
                        eprintln!("Available flags: --tokens, --ast, --help, --version");
                        eprintln!("Usage: pidgin-compiler <file.pg> [--tokens|--ast|--help|--version]");
                        std::process::exit(1);
                    }
            }
        }

        // Run the file if no flags were provided
        run_file(path);
    } else {
        run_prompt(); // If no file is given, start REPL prompt
    }
}

// Run a Pidgin source file
fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read file"); // Read file contents
    run(&source); // Run the source code
}

// Start a REPL (Read-Eval-Print Loop) prompt
fn run_prompt() {
    println!("Welcome to Pidgin REPL! Type 'exit' or 'quit' to exit, 'help' for help.");
    let mut interpreter = Interpreter::new(None); // Create a new interpreter
    let stdin = io::stdin(); // Get standard input
    let mut stdout = io::stdout(); // Get standard output
    loop {
        print!("pidgin> "); // Print prompt
        stdout.flush().unwrap(); // Flush output buffer
        let mut buffer = String::new(); // Buffer for user input
        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                println!("\nExiting...");
                break; // Exit on EOF
            }
            Ok(_) => {
                let input = buffer.trim();
                if input.is_empty() {
                    continue; // Skip empty lines
                }
                
                match input {
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    "clear" => {
                        // Clear screen (works on most terminals)
                        print!("\x1B[2J\x1B[1;1H");
                        stdout.flush().unwrap();
                        continue;
                    }
                    _ => {
                        if let Err(e) = run_with_interpreter(&buffer, &mut interpreter) {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

// Print help information for the REPL
fn print_help() {
    println!("Pidgin Compiler Usage:");
    println!("  pidgin-compiler <file.pg>              - Run a Pidgin program");
    println!("  pidgin-compiler <file.pg> --tokens     - Show tokens for a file");
    println!("  pidgin-compiler <file.pg> --ast        - Show AST for a file");
    println!("  pidgin-compiler <file.pg> --help       - Show this help message");
    println!("  pidgin-compiler <file.pg> --version    - Show version information");
    println!("  pidgin-compiler                         - Start interactive REPL");
    println!();
    println!("Pidgin REPL Commands:");
    println!("  exit, quit    - Exit the REPL");
    println!("  help          - Show this help message");
    println!("  clear         - Clear the screen");
    println!();
    println!("Pidgin Language Syntax:");
    println!("  let x = 10;           - Variable declaration");
    println!("  print x;              - Print a value");
    println!("  x = 20;               - Variable assignment");
    println!("  if (x > 5) {{ ... }}    - Conditional statement");
    println!("  while (x < 10) {{ ... }} - Loop statement");
    println!("  // comment            - Single-line comment");
    println!();
    println!("Examples:");
    println!("  let greeting = \"Hello, World!\";");
    println!("  print greeting;");
    println!("  let sum = 10 + 20;");
    println!("  print sum;");
}

// Run source code (used for files)
fn run(source: &str) {
    let mut interpreter = Interpreter::new(None); // Create a new interpreter
    if let Err(e) = run_with_interpreter(source, &mut interpreter) { // Run the code
        eprintln!("Error: {}", e); // Print error if any
    }
}

// Run source code with a given interpreter (used for REPL and files)
fn run_with_interpreter(source: &str, interpreter: &mut Interpreter) -> Result<(), String> {
    let mut lexer = lexer::Lexer::new(source); // Create a lexer
    let tokens = lexer.tokenize(); // Tokenize the source code
    let mut parser = parser::Parser::new(tokens.clone()); // Create a parser
    let program = parser.parse()?; // Parse tokens into AST
    interpreter.interpret(program, tokens) // Interpret the AST
}

// Display tokens for a given file
fn display_tokens(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read file"); // Read file contents
    let mut lexer = lexer::Lexer::new(&source); // Create a lexer
    let tokens = lexer.tokenize(); // Tokenize the source code
    for token in tokens {
        println!("{:?}", token); // Print each token
    }
}

// Display AST for a given file
fn display_ast(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read file"); // Read file contents
    let mut lexer = lexer::Lexer::new(&source); // Create a lexer
    let tokens = lexer.tokenize(); // Tokenize the source code
    let mut parser = parser::Parser::new(tokens); // Create a parser
    match parser.parse() {
        Ok(program) => println!("{:?}", program), // Print AST if parsing succeeds
        Err(e) => eprintln!("Parse error: {}", e), // Print error if parsing fails
    }
}

// Display the version of the compiler
fn display_version() {
    println!("Pidgin Compiler v{}", env!("CARGO_PKG_VERSION"));
    println!("Platform: {}", std::env::var("TARGET").unwrap_or_default());
    println!("Build Date: {}", std::env::var("VERGEN_BUILD_TIMESTAMP").unwrap_or_default());
}
