use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Let(String, Box<Expr>),
    Block(Vec<Expr>),
    FunctionCall(Vec<Expr>),
}
impl Expr {
    fn from_op(o: Operator, lhs: Expr, rhs: Option<Expr>) -> Option<Expr> {
        match o {
            Operator::ADD => Some(Expr::Add(Box::new(lhs), Box::new(rhs.unwrap()))),
            Operator::SUB => Some(Expr::Sub(Box::new(lhs), Box::new(rhs.unwrap()))),
            Operator::MUL => Some(Expr::Mul(Box::new(lhs), Box::new(rhs.unwrap()))),
            Operator::DIV => Some(Expr::Div(Box::new(lhs), Box::new(rhs.unwrap()))),
            Operator::CARAT => Some(Expr::Exp(Box::new(lhs), Box::new(rhs.unwrap()))),
            Operator::NEG => Some(Expr::Neg(Box::new(lhs))),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Operator {
    str: &'static str,
    precedence: i32,
    associativity: Associativity,
    arity: i32,
}
impl Operator {
    const ADD: Operator = Operator {
        str: "+",
        precedence: 40,
        associativity: Associativity::Left,
        arity: 2,
    };
    const SUB: Operator = Operator {
        str: "-",
        precedence: 40,
        associativity: Associativity::Left,
        arity: 2,
    };
    const MUL: Operator = Operator {
        str: "*",
        precedence: 50,
        associativity: Associativity::Left,
        arity: 2,
    };
    const DIV: Operator = Operator {
        str: "/",
        precedence: 50,
        associativity: Associativity::Left,
        arity: 2,
    };
    const CARAT: Operator = Operator {
        str: "^",
        precedence: 60,
        associativity: Associativity::Right,
        arity: 2,
    };
    const NEG: Operator = Operator {
        str: "-",
        precedence: 55,
        associativity: Associativity::Right,
        arity: 1,
    };
    const LPAREN: Operator = Operator {
        str: "(",
        precedence: 0,
        associativity: Associativity::Left,
        arity: 0,
    };

    pub fn from_token(t: Option<&Token>) -> Option<Operator> {
        if let None = t {
            return None;
        }
        let t = t.unwrap();
        match t {
            Token::Plus => Some(Operator::ADD),
            Token::Minus => Some(Operator::SUB),
            Token::Asterisk => Some(Operator::MUL),
            Token::Slash => Some(Operator::DIV),
            Token::Carat => Some(Operator::CARAT),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Associativity {
    Left,
    Right,
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut op_stack: Vec<Operator> = Vec::new();
    let mut expr_stack = Vec::new();

    let mut last_token_type: Option<&str> = None;

    let mut tokens = tokens.iter().peekable();

    'outer: while let Some(t) = tokens.next() {
        match t {
            Token::Number(n) => {
                expr_stack.push(Expr::Number(*n));
                last_token_type = Some("val");
            }
            Token::Identifier(i) => {
                expr_stack.push(Expr::Identifier(i.clone()));
                last_token_type = Some("val");
            }
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Carat => {
                if last_token_type == Some("val") {
                    while !op_stack.is_empty() {
                        let op = Operator::from_token(Some(t)).unwrap();
                        let last_op = op_stack.last().unwrap();

                        let this_op_precedence = op.precedence;
                        let last_op_precedence = last_op.precedence;

                        if this_op_precedence < last_op_precedence
                            || (this_op_precedence == last_op_precedence
                                && op.associativity == Associativity::Left)
                        {
                            println!("{:?}", expr_stack);
                            if op.arity == 1 || last_op.arity == 1 {
                                let val = expr_stack.pop().unwrap();
                                expr_stack.push(
                                    Expr::from_op(op_stack.pop().unwrap(), val, None).unwrap(),
                                );
                            } else {
                                let rhs = expr_stack.pop().unwrap();
                                let lhs = expr_stack.pop().unwrap();
                                expr_stack.push(
                                    Expr::from_op(op_stack.pop().unwrap(), lhs, Some(rhs)).unwrap(),
                                );
                            }
                        } else {
                            break;
                        }
                    }
                    op_stack.push(Operator::from_token(Some(t)).unwrap());
                } else {
                    let op = Operator::from_token(Some(t)).unwrap();
                    if op == Operator::SUB {
                        op_stack.push(Operator::NEG);
                    }
                }
                last_token_type = Some("op");
            }
            Token::LParen => {
                op_stack.push(Operator::LPAREN);
                last_token_type = Some("op");
            }
            Token::RParen => {
                while let Some(op) = op_stack.pop() {
                    if op == Operator::LPAREN {
                        break;
                    } else {
                        if op.arity == 1 {
                            let val = expr_stack.pop().unwrap();
                            expr_stack.push(Expr::from_op(op, val, None).unwrap());
                        } else {
                            let rhs = expr_stack.pop().unwrap();
                            let lhs = expr_stack.pop().unwrap();
                            expr_stack.push(Expr::from_op(op, lhs, Some(rhs)).unwrap());
                        }
                    }
                }
                last_token_type = Some("val");
            }
           
            _ => {}
        }
    }

    while !op_stack.is_empty() {
        if op_stack.last().unwrap().arity == 1 {
            let val = expr_stack.pop().unwrap();
            expr_stack.push(Expr::from_op(op_stack.pop().unwrap(), val, None).unwrap());
        } else {
            let rhs = expr_stack.pop().unwrap();
            let lhs = expr_stack.pop().unwrap();
            expr_stack.push(Expr::from_op(op_stack.pop().unwrap(), lhs, Some(rhs)).unwrap());
        }
    }

    println!("{:?}, {:?}", op_stack, expr_stack);

    expr_stack.pop().unwrap()
}

pub enum ParseError<'a> {
    ExpectedButGot(Token, Option<&'a Token>)
}

pub type IResult<I, O, E> = Result<(I, O), E>;

pub fn parse_let<'a>(input: Vec<Token>) -> IResult<Vec<Token>, Expr, ParseError<'a>> {
    let mut iter = input.iter();

    let first = iter.next();
    if let Some(t) = first {
        let second = iter.next();
        if let Some(Token::Identifier(i)) = second {
            let third = iter.next();
            if let Some(Token::Eq) = third {
                let (rest, expr) = parse_expr(input)?;

                return Ok((rest, expr))

            } else {
                return Err(ParseError::ExpectedButGot(Token::Eq, third))
            }
        } else {
            return Err(ParseError::ExpectedButGot(Token::Identifier(String::new()), second))
        }
    } else {
        return Err(ParseError::ExpectedButGot(Token::Let, first))
    }
}

pub fn parse_expr<'a>(tokens: Vec<Token>) -> IResult<Vec<Token>, Expr, ParseError<'a>> {
    todo!()
}