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

pub enum Alone {
    Int(i32);
    VarName(i32);
    Paren(Expr);
}


pub enum Alone {
    Int(i32),
    Variable(String),
    // need to parenthesis support later
}

pub enum MD_Expr {
    Mul {
        md_expr: MD_Expr,
        alone: Alone, 
    },
    Div {
        md_expr: MD_Expr,
        alone: Alone, 
    },
    alone(Alone),
}

pub enum Expr {
    Add {
        expr: Expr,
        md_expr: MD_Expr,
    },
    Subtract {
        expr: Expr,
        md_expr: MD_Expr,
    },
    md_expr(MD_Expr),
}


pub enum BoolExpr {
    // maybe add a bool here
    Comp {
        left: Expr,
        right: Expr,
        operator: Cond,
    }
}


pub enum Stmt {
    If {
        variable: String,
        expr: Expr,
    },
    While {
        condition: BoolExpr,
        body: Body,
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

pub struct Body { 
    pub body: Vec<Stmt>,
}

fn parse_alone(iter: &mut Peekable<Iter<Token>>) -> Alone {
    match iter.next() {
        Some(Token{token: TokenType::Decimal(x),..} => Alone::Int(x),
        Some(Token{token: TokenType::VarName(x),..} => Alone::VarName(x),
        None => {// shouldn't happen}
        /* need some 
         * Some(Token{token: TokenType::LParen}
        */ 
    }
}

fn parse_md_expr(iter: &mut Peekable<Iter<Token>>) -> MD_Expr {
    let the_alone: Alone = parse_alone(iter);
    match iter.next() {
        Some(Token{token: TokenType::Multiply,..} => {
            MD_Expr::Mul {
                md_expr: parse_md_expr(&mut iter),
                alone: the_alone, 
            }
        },
        Some(Token{token: TokenType::Divide,..} => {
            MD_Expr::Div {
                md_expr: parse_md_expr(&mut iter),
                alone: the_alone, 
            }
        },
        _ => {
            // the md_expr is only made of 1 alone thing:
            MD_Expr::alone(Alone);
        }
    }
}

fn parse_expr(&mut iter) -> Expr {
    let the_md_expr = parse_md_expr(&mut iter);

    match iter.peek() {
        Some(Token{token: TokenType::Add, ..}) => {
            iter.next();
            Expr::Add {
                expr: parse_expr(&mut iter),
                md_expr: the_md_expr,
            }
        },
        Some(Token{token: TokenType::Subtract, ..}) => {
            iter.next();
            Expr::Subtract {
                expr: parse_expr(&mut iter),
                md_expr: the_md_expr,
            }
        },
        _ => {
            Expr::md_expr(the_md_expr);
        }
    }

}

fn parse_boolexpr(iter: &mut Peekable<Iter<Token>>) -> BoolExpr {
    todo!();
}

fn get_var(iter: &mut Peekable<Iter<Token>>) -> String {
    match token.next() {
        Some(Token{token: TokenType::VarName(x)}) => s.clone();
        _ => {
            // shouldnt happen
            // todo!() error handling
        }
    }
}

fn parse_body(iter: &mut Peekable<Iter<Token>>) -> Body {
    let mut components = Vec![];
    match iter.next() {
        Some(Token{token: TokenType::EMERGE, ..} => {
            // this signals the start of the code and is good
        }
        Some(Token{token: TokenType::LBrace, ..} => {
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
    while(true) {
        match iter.next() {
            Some(Token {token: TokenType::Varinit, ..}) => {

            }
            Some(Token {token: TokenType::VarName, ..}) => {
                // assign
            }
            Some(Token {token: TokenType::If, ..}) => {

            }
            Some(Token {token: TokenType::While, ..}) => {

            }
            Some(Token {token: TokenType::Boom, ..}) => {

            }
        }
    }

}

pub fn make_ast(iter: &mut Peekable<Iter<Token>>) -> Body {
    let mut program = parse_body(&mut iter);
    let mut iter = tokens.iter().peekable();    
    return program;
}
