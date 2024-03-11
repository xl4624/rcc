mod ast;
mod lexer;
mod parser;
mod semantic_analyzer;
mod token;

use std::{error::Error, fs::read_to_string};

use crate::{lexer::lex, parser::parse, semantic_analyzer::analyze, token::Token};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let contents = read_to_string(&args[1])?;

    let tokens: Vec<Token> = lex(contents)?;
    tokens.iter().for_each(|t| println!("{:?}", t));

    let ast = parse(&tokens)?;
    println!("{:?}", ast);

    analyze(ast)?; // Check for semantic errors (undeclared variables or type mismatches)

    Ok(())
}
