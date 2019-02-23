use crate::scanner::scanner::Scanner;
use crate::parser::asp_stmt::AspStmt::Assignment;
use crate::parser::asp_stmt::AspStmt::ExprStmt;
use crate::parser::asp_assignment::AspAssignment;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::ParseError;

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

    // pub fn eval(&self) -> RuntimeValue {

    // }

    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        file.write_all(b"<AspStmt enum>\n");
        match self {
            // TODO
            // Possible to match multiple enums to do the same
            // thing? _(v) => v.pretty_print() ????
            Assignment(v) => v.test_parser(file, indentation + 1)?,
            ExprStmt(v)   => v.test_parser(file, indentation + 1)?,
        }
        file.write_all(b"<\\AspStmt enum\\>\n");
        Ok(())
    }
}
