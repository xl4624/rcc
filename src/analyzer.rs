use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{
    lexer::Type,
    parser::{Expression, Function, Program, Statement},
};

pub struct Analyzer {
    symbol_table: SymbolTable,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer { symbol_table: SymbolTable::new() }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<()> {
        for function in &program.functions {
            self.analyze_function(function)?;
        }
        Ok(())
    }

    fn analyze_function(&mut self, function: &Function) -> Result<()> {
        let info = SymbolInfo {
            symbol_type: SymbolType::Function { parameters: Vec::new() },
            data_type: function.return_type.clone(),
        };
        self.symbol_table.insert(function.name.clone(), info);
        self.symbol_table.enter_scope();
        for statement in &function.body {
            self.analyze_statement(statement, &function.return_type)?;
        }
        self.symbol_table.exit_scope();
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

    #[allow(unreachable_patterns)]
    fn analyze_expression(&mut self, expression: &Option<Expression>) -> Result<Type> {
        match expression {
            Some(Expression::IntLit(_)) => Ok(Type::Int),
            Some(Expression::FunctionCall { name }) => match self.symbol_table.get(name) {
                Some(symbol_info) => Ok(symbol_info.data_type.clone()),
                None => Err(anyhow!("Undefined function {}", name)),
            },
            Some(Expression::Binary { left, op: _, right }) => {
                let left_expression = Some((**left).clone());
                let right_expression = Some((**right).clone());
                let left_type = self.analyze_expression(&left_expression)?;
                let right_type = self.analyze_expression(&right_expression)?;
                if left_type != right_type {
                    return Err(anyhow!("Type mismatch: {:?} and {:?}", left_type, right_type));
                }
                Ok(left_type)
            }
            None => Ok(Type::Void),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
struct SymbolTable {
    stack: Vec<HashMap<String, SymbolInfo>>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable { stack: vec![HashMap::new()] }
    }

    fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.stack.pop();
    }

    fn insert(&mut self, name: String, symbol_info: SymbolInfo) {
        if let Some(scope) = self.stack.last_mut() {
            scope.insert(name, symbol_info);
        }
    }

    fn get(&mut self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.stack.iter().rev() {
            if let Some(symbol_info) = scope.get(name) {
                return Some(symbol_info);
            }
        }
        None
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct SymbolInfo {
    symbol_type: SymbolType,
    data_type: Type,
}

#[allow(dead_code)]
#[derive(Debug)]
enum SymbolType {
    Function { parameters: Vec<Type> },
    Variable,
}
