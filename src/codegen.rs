use std::io::Write;

use anyhow::{anyhow, Result};

use crate::parser::{Expression, Function, Program, Statement};

pub struct CodeGenerator<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(writer: &'a mut dyn Write) -> Self {
        CodeGenerator { writer }
    }

    pub fn generate(&mut self, program: &Program) -> Result<()> {
        for function in &program.functions {
            self.generate_function(function)?;
        }
        Ok(())
    }

    fn generate_function(&mut self, function: &Function) -> Result<()> {
        writeln!(self.writer, ".globl _{}", function.name)?;
        writeln!(self.writer, "_{}:", function.name)?;
        for statement in &function.body {
            self.generate_statement(statement)?;
        }
        Ok(())
    }

    fn generate_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Return(expression) => match expression {
                Some(Expression::IntLit(value)) => {
                    writeln!(self.writer, "mov w0, #{}", value)?;
                    writeln!(self.writer, "ret")?;
                }
                None => writeln!(self.writer, "ret")?,
                _ => return Err(anyhow!("Unsupported return expression")),
            },
            _ => return Err(anyhow!("Unsupported statement type")),
        }
        Ok(())
    }
}
