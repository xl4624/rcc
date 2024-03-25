mod analyzer;
mod codegen;
mod lexer;
mod parser;

use std::{
    fs::{read_to_string, File},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use lexer::Lexer;
use serde_json::to_string_pretty;

use crate::{analyzer::Analyzer, codegen::CodeGenerator};

#[derive(clap::Parser)]
#[clap(name = "rcc")]
#[clap(version, about, long_about = None)]
struct Args {
    /// The input file
    #[arg(value_name = "FILE.c", value_parser = is_c_file)]
    input_path: PathBuf,

    /// Print the output of each stage of the compiler
    #[arg(short, long)]
    print_output: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let contents = read_to_string(&args.input_path)?;
    let filename = args.input_path.to_string_lossy().to_string();

    let tokens = Lexer::new(filename, &contents).lex()?;
    let ast = parser::Parser::new(&tokens).parse()?;
    // Propagates semantic errors like undefined functions, unexpected return types, etc.
    Analyzer::new().analyze(&ast)?;

    let output_path = args.input_path.with_extension("s");
    let mut output_file = File::create(&output_path)?;
    CodeGenerator::new(&mut output_file).generate(&ast)?;

    if args.print_output {
        tokens.iter().for_each(|t| println!("{}", t));
        println!();
        println!("Program: {}", to_string_pretty(&ast)?);
        println!();
        println!("{}", read_to_string(&output_path)?);
    }

    Ok(())
}

fn is_c_file(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    if path.extension().unwrap_or_default() == "c" {
        Ok(path)
    } else {
        Err(anyhow!("Input file must have a .c extension"))
    }
}
