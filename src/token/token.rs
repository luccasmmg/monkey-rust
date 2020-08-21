#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,
    // Identifies + literals
    Ident(String),
    Int(i64),
    //Operators
    Assign,
    Plus,
    //Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    //Keyword
    Function,
    Let,
}

pub fn get_token(keyword: &str) -> Token {
    match keyword {
        "fn" => Token::Function,
        "let" => Token::Let,
        "=" => Token::Assign,
        "+" => Token::Plus,
        "," => Token::Comma,
        ";" => Token::Semicolon,
        "(" => Token::LParen,
        ")" => Token::RParen,
        "{" => Token::LBrace,
        "}" => Token::RBrace,
        &_ => match keyword.parse::<i64>() {
            Ok(t) => Token::Int(t),
            Err(_) => Token::Ident(keyword.to_string())
        }
    }
}
