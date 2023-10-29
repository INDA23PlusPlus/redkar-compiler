use crate::ast::{Condition, Alone, MD_Expr, Expr, BoolOp, BoolExpr, Stmt, Body};

// shouldnt work cause not private

fn transpile_condition(the_condition: &Condition, code: &mut String) {
    match the_condition {
        Condition::LessThan => {
            code += "<";
        }
        Condition::LessEqual => {
            code += "<=";
        }
        Condition::CheckEqual => {
            code += "<=";
        }
        Condition::NotEqual => {
            code += "!=";
        }
        Condition::True => {
            // shouldnt happen
            code += "true";
        }
        Condition::False => {
            // shouldnt happen
            code += "false";
        }
    }
}

fn transpile_alone(the_alone: &Stmt, code: &mut String) {
    match the_alone {
        Alone::Int(x) => {
            code += x.to_string().as_str();
        }
        Alone::Variable(s) => {
            code += s.as_str();
        }
    }
}

fn transpile_md_expr(the_mdexpr: &MD_Expr, code: &mut String) {
    match the_mdexpr {
        MD_Expr::Mul => {
            transpile_md_expr(&(*the_mdexpr.md_expr), &mut code);
            code += " * ";
            transpile_alone(&(*the_mdexpr.alone), &mut code);
        }
        MD_Expr::Div => {
            transpile_md_expr(&(*the_mdexpr.md_expr), code);
            code += " / ";
            transpile_alone(&(*the_mdexpr.alone), &mut code);
        }
        MD_Expr::alone(x) => {
            transpile_alone(&(*x), &mut code);
        }
    }
}

fn transpile_expr(the_expr: &Expr, code: &mut String) {
    match the_expr {
        Expr::Add => {
            transpile_expr(&(*the_expr.expr), &mut code);
            code += " + ";
            transpile_md_expr(&(*the_expr.md_expr), &mut code);
        }
        Expr::Subtract => {
            transpile_expr(&(*the_expr.expr), &mut code);
            code += " - ";
            transpile_md_expr(&(*the_expr.md_expr), &mut code);
        }
        Expr::md_expr(x) => {
            transpile_expr(&(*the_expr.expr), &mut code);
            // it is a md_expr
        }
    }
}

fn transpile_boolop(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_boolexpr(boolexpr: &BoolExpr, code: &mut String) {
    match boolexpr {
        BoolExpr::Comp => {
            transpile_expr(&boolexpr.left, code);
            transpile_expr(&boolexpr.right, code);
            transpile_condition(&boolexpr.operator, code);
        }
        BoolExpr::True => {
            code += "true";
        }
        BoolExpr::False => {
            code += "false";
        }
    }
}

fn transpile_statement(statement: &Stmt, code: &mut String) {
    match statement {
        Stmt::If => {
        }
        Stmt::While => {
        }
        Stmt::Varinit => {
            code += "int ";
            code += statement.name.as_str();
            code += " = ";
            transpile_expr(&statement.name.expr, &mut code);
        }
        Stmt::Assign => {
            code += statement.name.as_str();
            code += " = ";
            transpile_expr(&statement.name.expr, &mut code);
        }
        Stmt::Boom => {
            transpile(&statement.expr, &mut code)
        }
    }
}

fn transpile_body(body: Body, code: &mut String) {
    for statement in body.body {
        transpile_statement(&statement, &mut code);
    }
}

pub fn transpile_ast(AST: Body) -> String {
    let mut code: String = String::from("
    #include <bits/stdc++.h>\n
    using namespace std;\n
                                        ");
    /* 
    let mut two: String = String::from("barn");
    code = [code, two].join("");
    code += two.as_str();
    */ 
    transpile_body(AST, &mut code);

    return code;
} 
