use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Carat,
    Slash,
    LParen,
    RParen,

    String(String),

    Eq,

    Let,

    SemiColon,
}

const KEYWORDS: [&str; 1] = ["let"];

fn keyword(c: char, chars: &mut Peekable<Chars>) -> Option<Token> {
    for k in KEYWORDS {
        let mut kw_chars = k.chars().peekable();

        if kw_chars.next() != Some(c) {
            continue;
        }

        while let Some(c) = kw_chars.next() {
            if chars.peek() != Some(&c) {
                return None;
            }
            chars.next();
        }
        return match k {
            "let" => Some(Token::Let),
            _ => None,
        };
    }

    None
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            '^' => tokens.push(Token::Carat),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '=' => tokens.push(Token::Eq),
            ';' => tokens.push(Token::SemiColon),
            '"' => {
                let mut s = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    s.push(c);
                }
                tokens.push(Token::String(s));
            }
            _ => {
                if c.is_digit(10) || c == '.' {
                    let mut n = c.to_string();
                    while let Some(c2) = chars.peek() {
                        if c2.is_digit(10) || c2 == &'.' {
                            n.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(n.parse().unwrap()));
                } else if c.is_alphabetic() {
                    if let Some(k) = keyword(c, &mut chars) {
                        tokens.push(k);
                    } else {
                        let mut i = c.to_string();
                        while let Some(c2) = chars.peek() {
                            if c2.is_digit(10) {
                                i.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::Identifier(i));
                    }
                } else if c.is_whitespace() {
                    continue;
                } else {
                    panic!("unexpected character: {}", c);
                }
            }
        }
    }

    tokens
}
