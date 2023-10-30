use crate::ast::{Condition, Alone, MD_Expr, Expr, BoolOp, BoolExpr, Stmt, Body};

// shouldnt work cause not private

fn transpile_condition(the_condition: &Condition, code: &mut String) {
    match the_condition {
        Condition::LessThan => {
            code.push_str("<");
        }
        Condition::LessEqual => {
            code.push_str("<=");
        }
        Condition::CheckEqual => {
            code.push_str("<=");
        }
        Condition::NotEqual => {
            code.push_str("!=");
        }
        Condition::True => {
            // shouldnt happen
            code.push_str("true");
        }
        Condition::False => {
            // shouldnt happen
            code.push_str("false");
        }
    }
}

fn transpile_alone(the_alone: &Alone, code: &mut String) {
    match the_alone {
        Alone::Int(x) => {
            code.push_str(x.to_string().as_str());
        }
        Alone::Variable(s) => {
            code.push_str(s.as_str());
        }
    }
}

fn transpile_md_expr(the_mdexpr: &MD_Expr, code: &mut String) {
    match the_mdexpr {
        MD_Expr::Mul {md_expr: a, alone: b, } => {
            transpile_md_expr(&*a, code);
            code.push_str(" * ");
            transpile_alone(&*b, code);
        }
        MD_Expr::Div {md_expr: a, alone: b, } => {
            transpile_md_expr(&*a, code);
            code.push_str(" / ");
            transpile_alone(&*b, code);
        }
        MD_Expr::alone(x) => {
            transpile_alone(&*x, code);
        }
    }
}

fn transpile_expr(the_expr: &Expr, code: &mut String) {
    match the_expr {
        Expr::Add {expr: a, md_expr: b} => {
            transpile_expr(&*a, code);
            code.push_str(" + ");
            transpile_md_expr(&*b, code);
        }
        Expr::Subtract {expr: a, md_expr: b} => {
            transpile_expr(&*a, code);
            code.push_str(" - ");
            transpile_md_expr(&*b, code);
        }
        Expr::md_expr(x) => {
            transpile_md_expr(&*x, code);
            // it is a md_expr
        }
    }
}

fn transpile_boolop(statement: &Stmt, code: &mut String) {
    todo!();
}

fn transpile_boolexpr(boolexpr: &BoolExpr, code: &mut String) {
    match boolexpr {
        BoolExpr::Comp {
            left: l,
            right: r,
            operator: o,
        } => {
            transpile_expr(&l, code);
            transpile_condition(&o, code);
            transpile_expr(&r, code);
        }
        BoolExpr::True => {
            code.push_str("true");
        }
        BoolExpr::False => {
            code.push_str("false");
        }
    }
}

fn transpile_statement(statement: &Stmt, code: &mut String) {
    match statement {
        Stmt::If{condition: cond, body: bd, else_body: else_bd} => {
            code.push_str("if (");
            transpile_boolexpr(&cond, code);
            code.push(')');
            transpile_body(&(**bd), code);
            if else_bd.is_some() {
                transpile_body(&*(else_bd.as_ref().unwrap()), code);
            }
        }
        Stmt::While{condition: cond, body: bd} => {
            code.push_str("while (");
            transpile_boolexpr(&cond, code);
            code.push_str(") {\n    ");
            transpile_body(&*bd, code);
            code.push_str("}\n");
        }
        Stmt::Varinit{name: namn, expr: exp} => {
            code.push_str("int ");
            code.push_str(namn.as_str());
            code.push_str(" = ");
            transpile_expr(&exp, code);
            code.push_str(";\n");
        }
        Stmt::Assign{name: namn, expr: exp} => {
            code.push_str(namn.as_str());
            code.push_str(" = ");
            transpile_expr(&exp, code);
            code.push_str(";\n");
        }
        Stmt::Boom{expr: exp} => {
            code.push_str("cout << ");
            transpile_expr(&exp, code);
            code.push_str(" << \"\\n\"");
            code.push_str(";\n");
        }
    }
}

fn transpile_body(body: &Body, code: &mut String) {
    for statement in &body.body {
        transpile_statement(&statement, code);
    }
}

pub fn transpile_ast(AST: Body) -> String {
    let mut code: String = String::from("#include <bits/stdc++.h>\n\nusing namespace std;\n\nint main() {\n");
    /* 
    let mut two: String = String::from("barn");
    code = [code, two].join("");
    code.push_str(two.as_str();
    */ 
    transpile_body(&AST, &mut code);
    code.push_str("\n}");

    return code;
} 
