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
                    let mut i = c.to_string();
                    while let Some(c2) = chars.peek() {
                        if c2.is_digit(10) {
                            i.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(i));
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
