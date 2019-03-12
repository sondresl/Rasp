use crate::parser::asp_atom::AspAtom;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_arguments::AspArguments;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::RuntimeValue;
use crate::runtime::runtime::Scope;
use crate::log::logger::Logger;
use std::io;
use crate::parser::asp_and_test::AspAndTest;

#[derive(Debug,new)]
pub struct AspExpr {
    and_tests: Vec<AspAndTest>
}

impl AspExpr {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspExpr, AspParseError> {
        logger.enter_parser("AspExpr")?;

        let a = AspExpr::new(vec![AspAndTest::parse(sc, logger)?]);

        logger.leave_parser("AspExpr")?;
        Ok(a)
    }
}
