use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{
    lexer::Type,
    parser::{Expression, Function, Program, Statement},
};

pub struct Analyzer {
    symbol_table: HashMap<String, Type>,
}

#[allow(unreachable_code)]
impl Analyzer {
    pub fn new() -> Self {
        Analyzer { symbol_table: HashMap::new() }
    }

    pub fn analyze(&mut self, program: Program) -> Result<()> {
        for function in &program.functions {
            self.analyze_function(function)?;
        }
        Ok(())
    }

    fn analyze_function(&mut self, function: &Function) -> Result<()> {
        self.symbol_table.insert(function.name.clone(), function.return_type.clone());
        for statement in &function.body {
            self.analyze_statement(statement, &function.return_type)?;
        }
        Ok(())
    }

    fn analyze_statement(&mut self, statement: &Statement, return_type: &Type) -> Result<()> {
        match statement {
            Statement::Return(expression) => {
                let expression_type = self.analyze_expression(expression)?;
                if &expression_type != return_type {
                    return Err(anyhow!("Expected {:?}, found {:?}", return_type, expression_type));
                }
            }
        }
        Ok(())
    }

    fn analyze_expression(&mut self, expression: &Option<Expression>) -> Result<Type> {
        match expression {
            Some(Expression::IntLit(_)) => Ok(Type::Int),
            Some(Expression::FunctionCall { name }) => match self.symbol_table.get(name) {
                Some(typ) => Ok(typ.clone()),
                None => Err(anyhow!("Undefined function {}", name)),
            },
            None => Ok(Type::Void),
        }
    }
}
