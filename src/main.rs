mod lexer;
mod parser;
mod semantic_analyzer;

use std::{env::args, fs::read_to_string, process::exit};

use serde_json::to_string_pretty;

use crate::{
    lexer::{lex, Token},
    parser::{Parser, Program},
    semantic_analyzer::Analyzer,
};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        exit(1);
    }
    let contents: String = read_to_string(&args[1])?;

    let tokens: Vec<Token> = lex(contents)?;
    tokens.iter().for_each(|t| println!("{:?}", t));

    println!();
    let mut parser = Parser::new(&tokens);
    let ast: Program = parser.parse()?;
    println!("Program: {}", to_string_pretty(&ast)?);

    // Traverses through the AST and populates the symbol table. Alsochecks for semantic errors
    // like undefined functions, unexpected return types, etc.
    let mut analyzer = Analyzer::new();
    analyzer.analyze(ast)?;

    Ok(())
}
