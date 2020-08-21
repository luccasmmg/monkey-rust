use std::str;
use crate::token::token::{get_token, Token};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input,
            position: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = &self.input.as_bytes()[self.position as usize..];
        match remainder.len() {
            0 => None,
            _ => {
                if is_special(remainder[0] as char) {
                    self.position += 1;
                    Some(String::from_utf8(vec![remainder[0]]).unwrap())
                } else {
                    let string: Vec<u8> = remainder.iter().take_while(|x| !is_special(**x as char)).map(|x| *x).collect();
                    self.position += string.len();
                    Some(String::from_utf8((string.as_slice()).to_vec()).unwrap())
                }
            }
        }
    }
}

fn is_special(ch: char) -> bool {
    let special_chars: [char; 10] = ['=', ';', '(', ')', ',', '+', '{', '}', ' ', '\n'];
    special_chars.contains(&ch)
}

#[test]
fn it_works_iterator() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
";
    let letters: Vec<_> = Lexer::new(input).filter(|x| x != " " && x != "\n").collect();
    let test = vec!["let", "five", "=", "5", ";", "let",
                    "ten", "=", "10", ";", "let", "add", "=",
                    "fn", "(", "x", ",", "y", ")", "{", "x",
                    "+", "y", ";", "}", ";", "let", "result",
                    "=", "add", "(", "five", ",", "ten", ")", ";"
    ];
    assert_eq!(letters, test);
}

#[test]
fn it_works_with_tokens() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
";
    let letters: Vec<_> = Lexer::new(input).filter(|x| x != " " && x != "\n").map(|x| get_token(&x)).collect();
    let test = vec![Token::Let, Token::Ident(String::from("five")), Token::Assign, Token::Int(5), Token::Semicolon, Token::Let,
                    Token::Ident(String::from("ten")), Token::Assign, Token::Int(10), Token::Semicolon, Token::Let, Token::Ident(String::from("add")),
                    Token::Assign, Token::Function, Token::LParen, Token::Ident(String::from("x")), Token::Comma, Token::Ident(String::from("y")),
                    Token::RParen, Token::LBrace, Token::Ident(String::from("x")), Token::Plus, Token::Ident(String::from("y")), Token::Semicolon, Token::RBrace, Token::Semicolon,
                    Token::Let, Token::Ident(String::from("result")), Token::Assign, Token::Ident(String::from("add")),  Token::LParen, Token::Ident(String::from("five")),
                    Token::Comma, Token::Ident(String::from("ten")), Token::RParen, Token::Semicolon
    ];
    assert_eq!(letters, test);
}
