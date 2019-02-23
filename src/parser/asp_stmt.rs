use crate::scanner::scanner::Scanner;
use crate::parser::asp_stmt::AspStmt::Assignment;
use crate::parser::asp_stmt::AspStmt::ExprStmt;
use crate::parser::asp_assignment::AspAssignment;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::ParseError;
use crate::runtime::runtime::Scope;
use crate::runtime::runtime::RuntimeValue;


use std::fs::File;
use std::io::prelude::*;


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

    /// See asp_program.rs for early doc of eval.
    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        let rv = match self {
            Assignment(v) => v.eval(&mut cur_scope),
            ExprStmt(v)   => v.eval(&mut cur_scope),
        };
        rv
    }

    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        for _ in 0..(indentation * 2) { file.write(b" ")?; }
        file.write(b"<AspStmt enum>\n")?;

        match self {
            // TODO
            // Possible to match multiple enums to do the same
            // thing? _(v) => v.test_parser() ????
            Assignment(v) => v.test_parser(file, indentation + 1)?,
            ExprStmt(v)   => v.test_parser(file, indentation + 1)?,
        }

        for _ in 0..(indentation * 2) { file.write(b" ")?; }
        file.write(b"<AspStmt enum/>\n")?;
        Ok(())
    }
}
