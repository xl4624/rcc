mod analyzer;
mod codegen;
mod lexer;
mod parser;

use std::{
    env::args,
    fs::{read_to_string, File},
    path::Path,
    process::{self},
};

use serde_json::to_string_pretty;

use crate::{
    analyzer::Analyzer,
    codegen::CodeGenerator,
    lexer::{lex, Token},
    parser::Parser,
};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", &args[0]);
        process::exit(1);
    }
    let input_path = Path::new(&args[1]);
    if input_path.extension().unwrap_or_default() != "c" {
        eprintln!("Error: File must have a .c file extension");
        process::exit(1);
    }

    let contents = read_to_string(input_path)?;

    let tokens: Vec<Token> = lex(&contents)?;
    tokens.iter().for_each(|t| println!("{:?}", t));

    println!();
    let ast = Parser::new(&tokens).parse()?;
    println!("Program: {}", to_string_pretty(&ast)?);

    // Traverses through the AST and populates the symbol table. Also checks and propagates
    // semantic errors like undefined functions, unexpected return types, etc.
    Analyzer::new().analyze(&ast)?;

    let output_path = input_path.with_extension("s");
    let mut output_file = File::create(&output_path)?;
    CodeGenerator::new(&mut output_file).generate(&ast)?;

    println!("{}", read_to_string(&output_path)?);

    Ok(())
}
