use std::env;
use std::fs;
use crate::lexer::tokenize;
use crate::lexer::Token;

mod lexer;

fn main() {
    let file_path = "/home/ac41991/kth/redkar-compiler/src/source.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file!");

    let s = String::from(contents.clone());

    let tok: Vec<Token> = tokenize(s);
    dbg!(tok);

    println!("{contents}");

}
