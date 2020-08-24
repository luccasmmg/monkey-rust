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
                    if can_be_to_digit(remainder[0] as char) && (remainder[1] as char) == '=' {
                        self.position += 2;
                        return Some(String::from_utf8(vec![remainder[0], remainder[1]]).unwrap())
                    }
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

fn can_be_to_digit(ch: char) -> bool {
    let can_be_to_digit_chars: [char; 2] = ['=', '!'];
    can_be_to_digit_chars.contains(&ch)
}

pub fn convert_to_tokens(input: &str) -> Vec<Token> {
    Lexer::new(input).filter(|x| x != " " && x != "\n").map(|x| get_token(&x)).collect::<Vec<_>>()
}

#[test]
fn it_works_iterator() {
    let input = "let five == 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
";
    let letters = Lexer::new(input).filter(|x| x != " " && x != "\n").collect::<Vec<_>>();
    let test = vec!["let", "five", "==", "5", ";", "let",
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

#[test]
fn it_works_with_more() {
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
return true;
} else {
return false;
}
10 == 10;
10 != 9;
";
    let letters: Vec<_> = Lexer::new(input).filter(|x| x != " " && x != "\n").map(|x| get_token(&x)).collect();
    let test = vec![Token::Let, Token::Ident(String::from("five")), Token::Assign, Token::Int(5), Token::Semicolon, Token::Let, Token::Ident(String::from("ten")), Token::Assign, Token::Int(10), Token::Semicolon,
                    Token::Let, Token::Ident(String::from("add")), Token::Assign, Token::Function, Token::LParen, Token::Ident(String::from("x")), Token::Comma, Token::Ident(String::from("y")), Token::RParen,
                    Token::LBrace, Token::Ident(String::from("x")), Token::Plus, Token::Ident(String::from("y")), Token::Semicolon, Token::RBrace, Token::Semicolon, Token::Let, Token::Ident(String::from("result")),
                    Token::Assign, Token::Ident(String::from("add")), Token::LParen,
                    Token::Ident(String::from("five")), Token::Comma, Token::Ident(String::from("ten")), Token::RParen, Token::Semicolon, Token::Eof, Token::Semicolon, Token::Int(5), Token::LT, Token::Int(10), Token::GT, Token::Int(5),
                    Token::Semicolon, Token::If, Token::LParen, Token::Int(5), Token::LT, Token::Int(10), Token::RParen, Token::LBrace, Token::Return, Token::True, Token::Semicolon, Token::RBrace, Token::Else, Token::LBrace, Token::Return,
                    Token::False, Token::Semicolon, Token::RBrace, Token::Int(10), Token::Eq, Token::Int(10), Token::Semicolon, Token::Int(10), Token::Bang, Token::Assign, Token::Int(9), Token::Semicolon
    ];
    assert_eq!(letters, test);
}
