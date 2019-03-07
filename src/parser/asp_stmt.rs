use crate::scanner::scanner::Scanner;
use crate::parser::asp_stmt::AspStmt::Assignment;
use crate::parser::asp_stmt::AspStmt::ExprStmt;
use crate::parser::asp_assignment::AspAssignment;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::Scope;
use crate::runtime::runtime::RuntimeValue;
use crate::log::logger::Logger;
use std::io;


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
            logger.enter_parser("AspExprStmt")?;
            let a = ExprStmt(AspExpr::parse(sc, logger)?);
            logger.leave_parser("AspExprStmt")?;
            a
        };

        logger.leave_parser("AspStmt enum")?;

        Ok(asp_stmt)
    }

}
    // See asp_program.rs for early doc of eval.
//    pub fn eval(&self, mut cur_scope: &mut Scope) -> RuntimeValue {
//        let rv = match self {
//            Assignment(v) => v.eval(&mut cur_scope),
////            ExprStmt(v)   => v.eval(&mut cur_scope),
//        };
//        rv
//    }

//    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
//        logger.enter_parser("AspStmt enum")?;
//        match self {
//            // TODO
//            // Possible to match multiple enums to do the same
//            // thing? _(v) => v.test_parser() ????
////            Assignment(v) => v.test_parser(logger)?,
////            ExprStmt(v)   => v.test_parser(logger)?,
//        }
//        logger.leave_parser("AspStmt enum")
//    }
