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

pub enum AB {
    Mul {
        ab: AB,
        alone: Alone, 
    },
    Div {
        ab: AB,
        alone: Alone, 
    },
    alone(Alone);
}

pub enum Expr {
    Plus {
        expr: Expr,
        ab: AB,
    },
    Minus {
        expr: Expr,
        ab: AB,
    },
    ab(AB), 
}

pub enum BoolExpr {
    cond: Condition,
    expr1: Option<Expr>,
    expr2: Option<Expr>, 
}


pub enum Node_Type {
    Body,
    Assign,
    VarInit,
    While,
    If, 
    Else,
    BoolExpr(BoolExpr),
    Expr(Expr),
    AB(AB),
    Alone(Alone);, 
    Death,
}

pub struct Node {
    pub tp: Node_Type,
    pub kids: Vec<Node>,
}

// saving some death code 
/* 
Token {token: TokenType::Die, ..} => {
    node.kids.push(Node{tp: Node_Type::Death, kids: Vec![]});
    return;
},
*/

// I think this function can be private
fn ast_if(node: &mut Node, iter: &mut Peekable<Iter<Token>>) {

}

fn ast_expr(&mut Node, iter: &mut Peekable<Iter<Token>>) -> Node_Type::Expr {

}

fn ast_boolexpr (node: &mut Node, iter: &mut Peekable<Iter<Token>>) -> Node_Type::BoolExpr {

}

fn ast_AB (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}

fn ast_alone (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}

fn ast_expr (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}

fn ast_varinit (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}

fn ast_assign (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}

fn ast_boom (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}

fn ast_while (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}

fn ast_body (node: &mut Node, iter: &mut Peekable<Iter<Token>>) {
    todo!();
}


pub fn make_ast(tokens: Vec<Token>) -> Node {
    let mut start_node = Node {tp: Body, kids: Vec![] };
    let mut iter = tokens.iter().peekable();    
    ast_body(&mut start_node, &mut iter);
    return start_node;
}

