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
    Minus,
    Bang,
    Asterisk,
    Slash,
    Eq,
    NotEq,
    //Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LT,
    GT,
    //Keyword
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

pub fn get_token(keyword: &str) -> Token {
    match keyword {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        "==" => Token::Eq,
        "!=" => Token::NotEq,
        "=" => Token::Assign,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "!" => Token::Bang,
        "*" => Token::Asterisk,
        "/" => Token::Slash,
        "," => Token::Comma,
        ";" => Token::Semicolon,
        "(" => Token::LParen,
        ")" => Token::RParen,
        "{" => Token::LBrace,
        "}" => Token::RBrace,
        "<" => Token::LT,
        ">" => Token::GT,
        &_ => match keyword.parse::<i64>() {
            Ok(t) => Token::Int(t),
            Err(_) => match ident_is_invalid(keyword) {
                false => Token::Ident(keyword.to_string()),
                true => Token::Eof
            }
        }
    }
}

fn ident_is_invalid(ident: &str) -> bool {
   ident.as_bytes().iter().any(|x| !is_letter(*x as char))
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '!' || ch == '?'
}
