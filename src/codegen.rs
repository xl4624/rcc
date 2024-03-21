use std::io::Write;

use anyhow::Result;

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
            Statement::Return(expression) => {
                if let Some(expr) = expression {
                    self.generate_expression(expr)?;
                }
                writeln!(self.writer, "    ret")?;
            }
        }
        Ok(())
    }

    fn generate_expression(&mut self, expression: &Expression) -> Result<()> {
        match expression {
            Expression::IntLit(n) => {
                writeln!(self.writer, "    mov w0, #{}", n)?;
            }
            Expression::FunctionCall { name } => {
                // Allocate space on the stack and save on x29 and x30
                writeln!(self.writer, "    stp x29, x30, [sp, -16]!")?;
                // Set the frame pointer to the current stack pointer
                writeln!(self.writer, "    mov x29, sp")?;
                writeln!(self.writer, "    bl _{}", name)?;
                // Restore the frame pointer and the link register
                writeln!(self.writer, "    ldp x29, x30, [sp], 16")?;
            }
        }
        Ok(())
    }
}
