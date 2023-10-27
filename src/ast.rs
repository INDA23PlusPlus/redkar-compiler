#![warn(non_camel_case_types)]

use crate::lexer::{Token, TokenType};
use std::slice::Iter;
use std::iter::Peekable;

pub enum Condition {
    True,
    False,
    LessThan, 
    LessEqual,
    CheckEqual,
    NotEqual,
}

#[derive(Debug)]
pub enum Alone {
    Int(i32),
    Variable(String),
    // need to parenthesis support later
}

#[derive(Debug)]
pub enum MD_Expr {
    Mul {
        md_expr: Box<MD_Expr>,
        alone: Box<Alone>, 
    },
    Div {
        md_expr: Box<MD_Expr>,
        alone: Box<Alone>, 
    },
    alone(Box<Alone>),
}

#[derive(Debug)]
pub enum Expr {
    Add {
        expr: Box<Expr>,
        md_expr: Box<MD_Expr>,
    },
    Subtract {
        expr: Box<Expr>,
        md_expr: Box<MD_Expr>,
    },
    md_expr(Box<MD_Expr>),
}

#[derive(Debug)]
pub enum BoolOp {
    And,
    Or, 
}


#[derive(Debug)]
pub enum BoolExpr {
    // maybe add a bool here
    Comp {
        left: Expr,
        right: Expr,
        operator: BoolOp,
    },
    True,
    False,
}


#[derive(Debug)]
pub enum Stmt {
    If {
        condition: BoolExpr,
        body: Box<Body>,
        else_body: Option<Box<Body>>
    },
    While {
        condition: BoolExpr,
        body: Box<Body>,
    },
    Varinit {
        name: String,
        expr: Expr,
    },
    Assign {
        name: String,
        expr: String,
    },
    Boom {
        expr: Expr
    },
}

#[derive(Debug)]
pub struct Body { 
    pub body: Vec<Stmt>,
}

fn parse_alone(iter: &mut Peekable<Iter<Token>>) -> Option<Alone> {
    match iter.next() {
        Some(Token{token: TokenType::Decimal(x),..}) => Some(Alone::Int(*x)),
        Some(Token{token: TokenType::VarName(x),..}) => Some(Alone::Variable(x.clone())),
        Some(other_token) => { None
            // shouldn't happen}
        /* need some 
         * Some(Token{token: TokenType::LParen}
        */ 
        },
        None => None,
        // todo!(): need some expr support here
    }
}

fn parse_md_expr(iter: &mut Peekable<Iter<Token>>) -> MD_Expr {
    let the_alone: Alone = parse_alone(iter).expect("There should be alone here");
    match iter.next() {
        Some(Token{token: TokenType::Multiply,..}) => {
            MD_Expr::Mul {
                md_expr: Box::new(parse_md_expr(iter)),
                alone: Box::new(the_alone), 
            }
        },
        Some(Token{token: TokenType::Divide,..}) => {
            MD_Expr::Div {
                md_expr: Box::new(parse_md_expr(iter)),
                alone: Box::new(the_alone), 
            }
        },
        _ => {
            // the md_expr is only made of 1 alone thing:
            MD_Expr::alone(Box::new(the_alone))
        }
    }
}

fn parse_expr(iter: &mut Peekable<Iter<Token>>) -> Expr {
    let the_md_expr = parse_md_expr(iter);

    match iter.peek() {
        Some(Token{token: TokenType::Add, ..}) => {
            iter.next();
            Expr::Add {
                expr: Box::new(parse_expr(iter)),
                md_expr: Box::new(the_md_expr),
            }
        },
        Some(Token{token: TokenType::Subtract, ..}) => {
            iter.next();
            Expr::Subtract {
                expr: Box::new(parse_expr(iter)),
                md_expr: Box::new(the_md_expr),
            }
        },
        _ => {
            Expr::md_expr(Box::new(the_md_expr))
        }
    }

}

fn parse_boolexpr(iter: &mut Peekable<Iter<Token>>) -> Option<BoolExpr> {
    match iter.peek() {
        Some(Token{token: TokenType::LParen, ..}) => {
            let x = iter.next();

            let boolexpr = parse_boolexpr(iter).expect("probably didnt find Rparen");
            match iter.next() {
                Some(Token{token: TokenType::RParen, ..}) => {Some(boolexpr)}
                _ => {
                    None
                    // some error
                }
            }
        }
        Some(Token{token: TokenType::True, ..}) => {
            Some(BoolExpr::True)
        }
        Some(Token{token: TokenType::False, ..}) => {
            Some(BoolExpr::False)
        }
        Some(other_token) => {
            let one = parse_expr(iter);
            // let op = iter.next().expect("need boolop");
            // let op = BoolOp::try_from(iter.next().expect("need boolop").clone());
            let op = iter.next();
            // assert!(Some(op.clone()) == Some(BoolOp));
            Some(BoolExpr::Comp {
                left: one,
                right: parse_expr(iter),
                operator: match op {
                    Some(Token{token: TokenType::And,..}) => BoolOp::And,
                    Some(Token{token: TokenType::Or,..}) => BoolOp::Or,
                    _ => {
                        return None; 
                    },
                },
            })
        }
        None => {
            None
            // todo!(): error shouldnt happen
        }
    }
}

fn get_var(iter: &mut Peekable<Iter<Token>>) -> Option<String> {
    match iter.next() {
        Some(Token{token: TokenType::VarName(x),..}) => Some(x.clone()),
        _ => {
            None
            // shouldnt happen
            // todo!() error handling
        }
    }
}

fn parse_body(iter: &mut Peekable<Iter<Token>>) -> Box<Body> {
    let mut parts = vec![];
    match iter.next() {
        Some(Token{token: TokenType::Emerge, ..}) => {
            // this signals the start of the code and is good
        }
        Some(Token{token: TokenType::LBrace, ..}) => {
            // this signals the start of the code and is good
        }
        Some(other_token) => {
            // this was not expected and should return an error
            // todo!() error handling
        }
        None => {
            // this technically means the body hasnt start? 
            // not sure if this would ever even happen should be an error
        }
    }

    
    // this should continue until it cant anymore
    while true {
        match iter.next() {
            Some(Token {token: TokenType::Var, ..}) => {
                let var: String = get_var(iter).expect("should have a VarName");
                match iter.next() {
                    Some(Token{token: TokenType::AssignEqual, ..}) => {
                        //good 
                    },
                    _ => {
                        // else error. There should always be an equals sign
                    },
                }
                parts.push(Stmt::Varinit {
                    name: var,
                    expr: parse_expr(iter),
                });
                match iter.next() {
                    Some(Token {token: TokenType::Semicolon, ..}) => {},
                    _ => {
                        // else error. There should always be an equals sign
                    },
                }
            },
            Some(Token {token: TokenType::VarName(x), ..}) => {
                // assign
                match iter.next() {
                    Some(Token{token: TokenType::AssignEqual, ..}) => {
                        //good 
                    },
                    _ => {
                        // else error. There should always be an equals sign
                    },
                }
                parts.push(Stmt::Varinit {
                    name: x.to_string(),
                    expr: parse_expr(iter),
                });
                match iter.next() {
                    Some(Token {token: TokenType::Semicolon, ..}) => {},
                    _ => {
                        // else error. There should always be an equals sign
                    },
                }
            },
            Some(Token {token: TokenType::If, ..}) => {
                match iter.next() {
                    Some(Token{token: TokenType::LParen, ..}) => {
                        //good 
                    },
                    _ => {
                        // else todo!() error. There should always be an equals sign
                    },
                }
                let cond: BoolExpr = parse_boolexpr(iter).expect("wanted boolexpr, didnt get");
                match iter.next() {
                    Some(Token{token: TokenType::RParen, ..}) => {
                        //good 
                    },
                    _ => {
                        // else todo!() error. There should always be an equals sign
                    },
                }
                let the_body = parse_body(iter);
                parts.push(Stmt::If {
                    condition: cond,
                    body: the_body,
                    else_body: {
                        // check this later, maybe should be iter.next instead of peek?
                        if let Some(Token{token: TokenType::Else, ..}) = iter.peek() {
                            Some(parse_body(iter))
                        }
                        else {
                            None
                        }
                    },
                });
                
            },
            Some(Token {token: TokenType::While, ..}) => {
                match iter.next() {
                    Some(Token{token: TokenType::LParen, ..}) => {
                        //good 
                    },
                    _ => {
                        // else todo!() error. There should always be an equals sign
                    },
                }
                let cond: BoolExpr = parse_boolexpr(iter).expect("wanted boolexpr, didnt get");
                match iter.next() {
                    Some(Token{token: TokenType::RParen, ..}) => {
                        //good 
                    },
                    _ => {
                        // else todo!() error. There should always be an equals sign
                    },
                }
                let the_body = parse_body(iter);
                parts.push(Stmt::While {
                    condition: cond,
                    body: the_body,
                });
            },
            Some(Token {token: TokenType::Boom, ..}) => {
                match iter.next() {
                    Some(Token{token: TokenType::LParen, ..}) => {
                        //good 
                    },
                    _ => {
                        // else todo!() error. There should always be an equals sign
                    },
                }
                // we dont really need to care what the expression is
                // since we are transpiling lol
                parts.push(Stmt::Boom {
                    expr: parse_expr(iter),
                });
                match iter.next() {
                    Some(Token{token: TokenType::RParen, ..}) => {},
                    _ => {
                        // else todo!() error. There should always be an equals sign
                    },
                }
                match iter.next() {
                    Some(Token{token: TokenType::Semicolon, ..}) => {},
                    _ => {
                        // else todo!() error. There should always be an equals sign
                    },
                }
            },
            Some(Token {token: TokenType::RBrace, ..}) => {
                break;
            },
            Some(Token {token: TokenType::Die, ..}) => {
                break;
            },
            Some(other_token) => {
                // bad, this is wrong syntax
            }
            None => {
                break;
            }
        }
    }
    return Box::new(Body{body: parts});

}

pub fn make_ast(tokens: Vec<Token>) -> Body {
    let mut iter = tokens.iter().peekable();    
    let mut program = *parse_body(&mut iter);
    return program;
}
