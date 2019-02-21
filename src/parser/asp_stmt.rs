use crate::scanner::scanner::Scanner;
use crate::parser::asp_stmt::AspStmt::Assignment;
use crate::parser::asp_stmt::AspStmt::ExprStmt;
use crate::parser::asp_assignment::AspAssignment;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::ParseError;


#[derive(Debug)]
pub enum AspStmt {
    Assignment(AspAssignment),
    ExprStmt(AspExpr)
}

impl AspStmt {
    pub fn parse(sc: &mut Scanner) -> Result<AspStmt,ParseError> {
        if sc.has_equal_token() {
            return Ok(Assignment(AspAssignment::parse(sc)?));
        }
        Ok(ExprStmt(AspExpr::parse(sc)?))
    }
}
