use std::env;
use std::fs;
use crate::tokenizer::tokenize;
use crate::tokenizer::Token;

mod tokenizer;
mod ast;
mod transpiler;

fn main() {
    let file_path = "/home/ac41991/kth/redkar-compiler/src/fib.raunak";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file!");

    let s = String::from(contents.clone());

    let tok: Vec<Token> = tokenize(s);
    dbg!(tok.clone());

    let abstract_syntax_tree = ast::make_ast(tok.clone());

    println!("\nThe source code is:\n{}", contents);

    let cpp = transpiler::transpile_ast(abstract_syntax_tree.clone());
    println!("{}", cpp);
   
    

}
