//! Token definitions for the AI-Lang lexer.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // literals
    Int(i64),
    Float(f64),
    String(String),
    Ident(String),
    // keywords
    Let,
    Print,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    Import,
    Grad,
    Dual,
    // operators / punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    // special
    Eof,
}

impl Token {
    pub fn is_keyword(s: &str) -> Option<Token> {
        match s {
            "let" => Some(Token::Let),
            "print" => Some(Token::Print),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
            "import" => Some(Token::Import),
            "grad" => Some(Token::Grad),
            "dual" => Some(Token::Dual),
            _ => None,
        }
    }
}
