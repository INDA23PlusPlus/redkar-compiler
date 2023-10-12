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

pub enum Node_Type {
    Body,
    Assign,
    VarInit,
    While,
    If, 
    Else,
    BoolExpr {
        cond: Condition, 
    }
    Expr,
    AB,
    Alone, 
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
    iter.next();
    match iter.peek() {
        Token {token: TokenType::True, ..} => {
            node.kids.push(Node {tp: Node_Type::BoolExpr{cond: Condition::True}, kids: vec![]});
        },
        Token {token: TokenType::False, ..} => {
            node.kids.push(Node {tp: Node_Type::BoolExpr{cond: Condition::False}, kids: vec![]});
        },
        Token{token: TokenType::NotEqual, ..} => {
            node.kids.push(Node{tp: Node_Type::BookExpr{cond: Condition::NotEqual}, kids: vec![]});
        }
        _ => {
            // is an expression
        }
    }
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

pub fn make_ast(tokens: Vec<Token>) -> Node {
    let mut start_node = Node {tp: Body, kids: Vec![] };
    let mut iter = tokens.iter().peekable();    
}

