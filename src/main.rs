mod codegen;
mod lexer;
mod parser;

use std::{env::args, error::Error, fs};

use crate::{
    codegen::generate_assembly,
    lexer::{lex, Token},
    parser::{parse, Program},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename: &str = &args[1];
    let input = std::fs::read_to_string(filename)?;

    let tokens: Vec<Token> = lex(&input)?;
    tokens.iter().for_each(|t| println!("{}", t));

    let root: Program = parse(&tokens)?;
    println!("Root: {:?}", root);

    let assembly = generate_assembly(&root)?;
    println!("{}", assembly);

    let output_filename = format!("{}.s", filename.trim_end_matches(".c"));
    fs::write(output_filename, assembly)?;

    Ok(())
}
