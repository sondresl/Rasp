use crate::scanner::scanner::Scanner;
use crate::parser::asp_stmt::AspStmt::Assignment;
use crate::parser::asp_stmt::AspStmt::ExprStmt;
use crate::parser::asp_assignment::AspAssignment;
use crate::parser::asp_expr::AspExpr;
use crate::parser::asp_syntax::AspSyntax;

#[derive(Debug)]
pub enum AspStmt {
    Assignment(AspAssignment),
    ExprStmt(AspExpr)
}

impl AspSyntax for AspStmt {
    fn parse(sc: &mut Scanner) -> AspStmt {
        if sc.has_equal_token() {
            return Assignment(AspAssignment::parse(sc));
        }
        return ExprStmt(AspExpr::parse(sc));
    }
}
