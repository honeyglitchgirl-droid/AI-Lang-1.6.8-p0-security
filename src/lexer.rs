//! Lexer for AI-Lang.
//!
//! Turns source text into a stream of tokens. Handles integers, floats,
//! strings, identifiers, keywords, and operators. Reports lex errors with
//! position information.

use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lex error at {}:{}: {}", self.line, self.col, self.message)
    }
}

impl std::error::Error for LexError {}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            if self.chars.peek().is_none() {
                tokens.push(Token::Eof);
                break;
            }
            tokens.push(self.next_token()?);
        }
        Ok(tokens)
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.bump();
                }
                '\n' => {
                    self.bump();
                    self.line += 1;
                    self.col = 1;
                }
                '#' => {
                    // line comment
                    while let Some(&c) = self.chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.bump();
                    }
                }
                _ => break,
            }
        }
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.col += 1;
        }
        c
    }

    fn next_token(&mut self) -> Result<Token, LexError> {
        let c = match self.chars.peek().copied() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        match c {
            '+' => { self.bump(); Ok(Token::Plus) }
            '-' => { self.bump(); Ok(Token::Minus) }
            '*' => { self.bump(); Ok(Token::Star) }
            '/' => { self.bump(); Ok(Token::Slash) }
            '(' => { self.bump(); Ok(Token::LParen) }
            ')' => { self.bump(); Ok(Token::RParen) }
            '[' => { self.bump(); Ok(Token::LBracket) }
            ']' => { self.bump(); Ok(Token::RBracket) }
            '{' => { self.bump(); Ok(Token::LBrace) }
            '}' => { self.bump(); Ok(Token::RBrace) }
            ',' => { self.bump(); Ok(Token::Comma) }
            ';' => { self.bump(); Ok(Token::Semicolon) }
            '=' => {
                self.bump();
                if self.chars.peek() == Some(&'=') {
                    self.bump();
                    Ok(Token::EqEq)
                } else {
                    Ok(Token::Eq)
                }
            }
            '!' => {
                self.bump();
                if self.chars.peek() == Some(&'=') {
                    self.bump();
                    Ok(Token::NotEq)
                } else {
                    Err(LexError {
                        message: "unexpected '!'".into(),
                        line: self.line,
                        col: self.col,
                    })
                }
            }
            '<' => { self.bump(); Ok(Token::Lt) }
            '>' => { self.bump(); Ok(Token::Gt) }
            '"' => self.string_literal(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.ident_or_keyword(),
            other => Err(LexError {
                message: format!("unexpected character {other:?}"),
                line: self.line,
                col: self.col,
            }),
        }
    }

    fn string_literal(&mut self) -> Result<Token, LexError> {
        self.bump(); // opening "
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                self.bump();
                return Ok(Token::String(s));
            }
            if c == '\n' {
                return Err(LexError {
                    message: "unterminated string".into(),
                    line: self.line,
                    col: self.col,
                });
            }
            s.push(self.bump().unwrap());
        }
        Err(LexError {
            message: "unterminated string".into(),
            line: self.line,
            col: self.col,
        })
    }

    fn number(&mut self) -> Result<Token, LexError> {
        let mut s = String::new();
        let mut is_float = false;
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                s.push(self.bump().unwrap());
            } else if c == '.' && !is_float {
                is_float = true;
                s.push(self.bump().unwrap());
            } else {
                break;
            }
        }
        if is_float {
            s.parse::<f64>()
                .map(Token::Float)
                .map_err(|_| LexError {
                    message: format!("invalid float {s}"),
                    line: self.line,
                    col: self.col,
                })
        } else {
            s.parse::<i64>()
                .map(Token::Int)
                .map_err(|_| LexError {
                    message: format!("invalid integer {s}"),
                    line: self.line,
                    col: self.col,
                })
        }
    }

    fn ident_or_keyword(&mut self) -> Result<Token, LexError> {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                s.push(self.bump().unwrap());
            } else {
                break;
            }
        }
        Ok(Token::is_keyword(&s).unwrap_or(Token::Ident(s)))
    }
}
