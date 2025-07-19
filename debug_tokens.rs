// Let's create a simple program to see how tokenization works
use std::env;

mod token;
mod lexer;

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <code>", args[0]);
        return;
    }
    
    let source = &args[1];
    println!("Source code: {}", source);
    println!("Tokens:");
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?}", i, token);
    }
}
