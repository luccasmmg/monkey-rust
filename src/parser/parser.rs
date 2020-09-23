use crate::lexer::lexer::{Lexer, convert_to_tokens};
use crate::token::token::Token;
use crate::ast::ast::{ Statement, Expression, Literal, Identifier };

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: convert_to_tokens(input),
            position: 0
        }
    }
}

impl Iterator for Parser {
    type Item = Statement;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = &self.tokens[self.position..];
        println!("Remainder - {:?}", remainder);
        match remainder {
            [] => None,
            [Token::Let, end @ ..] => {
                let (stmt, n_position) = check_let(end);
                self.position += n_position;
                Some(stmt)
            },
            [Token::Return, end @ ..] => {
                let (stmt, n_position) = parse_return(end);
                self.position += n_position;
                Some(stmt)
            },
            [end @ ..] => {
                let (stmt, n_position) = parse_expression(end);
                self.position += n_position;
                Some(Statement::ExpressionStatement(stmt))
            },
        }
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

fn check_let(rest: &[Token]) -> (Statement, usize) {
    match rest[1] {
        Token::Assign => parse_let(rest),
        _ => panic!("Expected a LET TOKEN"),
    }
}

fn parse_return(rest: &[Token]) -> (Statement, usize) {
    let (value, position_up_expression) = parse_expression(&rest);
    (Statement::ReturnStatement(value), position_up_expression + 1)
}

fn parse_let(rest: &[Token]) -> (Statement, usize) {
    let identifier = parse_identifier(&rest[0]);
    let position_up_identifier = 2;
    let (value, position_up_expression) = parse_expression(&rest[position_up_identifier..]);
    (Statement::LetStatement(identifier, value),
     position_up_identifier + position_up_expression + 1)
}

fn parse_identifier(ident: &Token) -> Identifier {
    match ident {
        Token::Ident(s) => Identifier(String::from(s)),
        _ => panic!("Expected a IDENTIFIER"),
    }
}

fn parse_expression(exp: &[Token]) -> (Expression, usize) {
    match exp {
        [Token::Int(i), Token::Semicolon, ..] => (Expression::LiteralExp(Literal::IntLiteral(*i)), 2),
        [Token::Ident(i), Token::Semicolon, ..] => (Expression::IdentExp(Identifier(String::from(i))), 2),
        _ => panic!(),
    }
}

#[test]
fn test_let_statement() {
    let input = "let fivee = 5;";
    let parser = Parser::new(input).collect::<Vec<_>>();
    let test = vec![Statement::LetStatement(Identifier(String::from("fivee")), Expression::LiteralExp(Literal::IntLiteral(5)))
                    ];
    assert_eq!(test, parser);
}

#[test]
fn test_return_statement() {
    let input = "return 5; return 10;";
    let parser = Parser::new(input).collect::<Vec<_>>();
    let test = vec![Statement::ReturnStatement(Expression::LiteralExp(Literal::IntLiteral(5))),
                    Statement::ReturnStatement(Expression::LiteralExp(Literal::IntLiteral(10)))];
    assert_eq!(test, parser);
}

#[test]
fn test_return_and_let_statement() {
    let input = "let fivee = 5; let three = 3; return 5; return 10;";
    let parser = Parser::new(input).collect::<Vec<_>>();
    let test = vec![Statement::LetStatement(Identifier(String::from("fivee")), Expression::LiteralExp(Literal::IntLiteral(5))),
                    Statement::LetStatement(Identifier(String::from("three")), Expression::LiteralExp(Literal::IntLiteral(3))),
                    Statement::ReturnStatement(Expression::LiteralExp(Literal::IntLiteral(5))),
                    Statement::ReturnStatement(Expression::LiteralExp(Literal::IntLiteral(10)))];
    assert_eq!(test, parser);
}

#[test]
fn test_exp_statement_ident() {
    let input = "foobar;";
    let parser = Parser::new(input).collect::<Vec<_>>();
    let test = vec![Statement::ExpressionStatement(Expression::IdentExp(Identifier(String::from("foobar"))))];
    assert_eq!(test, parser);
}

#[test]
fn test_exp_statement_int() {
    let input = "5;";
    let parser = Parser::new(input).collect::<Vec<_>>();
    let test = vec![Statement::ExpressionStatement(Expression::LiteralExp(Literal::IntLiteral(5)))];
    assert_eq!(test, parser);
}
//ITERATOR RETURN STATEMENT
// IF CURRENT TOKEN = LET -> PARSE LET
// IF NEXT TOKEN != IDENT PANIC
// ELSE PARSE IDENTIFIER
// IF NEXT TOKEN != EQUAL TOKEN PANIC
// PORRA DE VARIABLE STATEMENT

// PARSE IDENTIFIER
// ALGUMA GOISA DE CRIAR AST NODE

// PARSE EXPRESSION
// IF CURRENT EQUAL INTEGER
// YOU CAN PARSE OPERATOR
// OR
// YOU CAN PARSE INTEGER LITERAL
// ELSE
// YOU CAN PARSE GROUPED EXPRESSION
