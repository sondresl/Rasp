use crate::scanner::scanner::Scanner;
use crate::parser::asp_stmt::AspStmt::Assignment;
use crate::parser::asp_stmt::AspStmt::ExprStmt;
use crate::parser::asp_assignment::AspAssignment;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::Scope;
use crate::runtime::runtime::RuntimeValue;
use crate::log::logger::Logger;
use crate::scanner::token::Token;


#[derive(Debug)]
pub enum AspStmt {
    Assignment(AspAssignment),
    ExprStmt(AspExpr)
}

impl AspStmt {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspStmt, AspParseError> {

        logger.enter_parser("AspStmt enum")?;

        let asp_stmt = if sc.has_equal_token() {
            Assignment(AspAssignment::parse(sc, logger)?)
        } else {
            match sc.cur_token() {
                // ExprStmt
                _ => {
                    logger.enter_parser("AspExprStmt")?;
                    let a = ExprStmt(AspExpr::parse(sc, logger)?);
                    sc.skip(Token::Newline)?;
                    logger.leave_parser("AspExprStmt")?;
                    a
                }
            }
        };

        logger.leave_parser("AspStmt enum")?;

        Ok(asp_stmt)
    }

    pub fn eval(&self, mut cur_scope: &mut Scope) -> RuntimeValue {
        match self {
            Assignment(v) => {
                v.eval(&mut cur_scope);
                RuntimeValue::RuntimeNone
            },
            ExprStmt(v)   => v.eval(&mut cur_scope),
        }
    }

}

