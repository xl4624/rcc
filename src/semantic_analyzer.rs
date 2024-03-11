use std::{collections::HashMap, error::Error};

use crate::{
    ast::{Expression, Function, Program, Statement},
    token::Type,
};

struct SymbolTable {
    table: HashMap<String, Type>,
}

#[allow(dead_code)]
impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { table: HashMap::new() }
    }

    pub fn insert(&mut self, name: String, ty: Type) {
        self.table.insert(name, ty);
    }

    pub fn remove(&mut self, name: &str) {
        self.table.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.table.get(name)
    }
}

pub fn analyze(program: Program) -> Result<(), Box<dyn Error>> {
    let mut symbol_table = SymbolTable::new();
    analyze_function(&program.function, &mut symbol_table)?;
    Ok(())
}

fn analyze_function(
    function: &Function,
    symbol_table: &mut SymbolTable,
) -> Result<(), Box<dyn Error>> {
    symbol_table.insert(function.name.clone(), function.ty.clone());
    for statement in &function.body {
        analyze_statement(statement, symbol_table, &function.ty)?;
    }
    Ok(())
}

fn analyze_statement(
    statement: &Statement,
    symbol_table: &mut SymbolTable,
    expected_return_type: &Type,
) -> Result<(), Box<dyn Error>> {
    match statement {
        Statement::Return(expression) => {
            let expression_type = analyze_expression(expression, symbol_table)?;
            if &expression_type != expected_return_type {
                return Err(format!(
                    "Expected return type {:?}, found {:?}",
                    expected_return_type, expression_type
                )
                .into());
            }
        }
    }
    Ok(())
}

fn analyze_expression(
    expression: &Option<Expression>,
    _symbol_table: &mut SymbolTable,
) -> Result<Type, Box<dyn Error>> {
    match expression {
        Some(Expression::IntLit(_)) => Ok(Type::Int),
        None => Ok(Type::Void),
        _ => Err("Unsupported expression".into()),
    }
}
