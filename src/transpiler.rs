use crate::ast::{Condition, Alone, MD_Expr, Expr, BoolOp, BoolExpr, Stmt, Body};

// shouldnt work cause not private

fn transpile_Alone(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_MD_Expr(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_Expr(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_boolop(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_boolexpr(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_statement(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_body(body: Body, code: &mut String) {
    for statement in body.body {
        transpile_statement(&statement, code);
    }
}

pub fn transpile_ast(AST: Body) -> String {
    let mut code: String = String::from("");
    /* 
    let mut two: String = String::from("barn");
    code = [code, two].join("");
    */ 
    transpile_body(AST, &mut code);

    return code;

}
