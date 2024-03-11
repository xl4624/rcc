use crate::token::Type;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Return(Option<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    IntLit(u64),
}
