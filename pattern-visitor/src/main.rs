use crate::syntax::visit::walk_expr;

// The data we will visit
/*
mod ast {
    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

// The abstract visitor
mod visit {
    use crate::ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use visit::*;
use ast::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 { panic!() }
    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}
*/

struct Vis();

impl syntax::visit::Visitor for Vis {}

mod syntax;
fn main() {
    let parse_result = syntax::parse("10 * 20 + 3");
    let mut vis = Vis();

    match parse_result {
        Ok(expr) => {
            println!(
                "result is : {:?}",
                syntax::visit::walk_expr(&mut vis, &expr)
            );
        }
        Err(e) => {
            println!("failed when parse expr, error is : {}", e)
        }
    }

    //let i: i64 = 12;
    //let data = Expr::IntLit(i);
    //let visitor = Interpreter::visit_expr(&mut self, &data);
}
