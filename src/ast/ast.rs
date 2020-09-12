use std::fmt;
use crate::lexer::Token;

struct Program {
    statements: Vec<Statement>
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Statement {
    LetStatement(Identifier, Expression)
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Identifier(pub String);
