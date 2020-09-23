use std::fmt;
use crate::token::token::Token;

struct Program {
    statements: Vec<Statement>
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression)
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Identifier(pub String);

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum Expression {
    IdentExpression(Identifier),
    LiteralExp(Literal),
}
