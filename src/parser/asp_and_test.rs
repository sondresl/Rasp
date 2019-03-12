use crate::parser::asp_not_test::AspNotTest;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::{RuntimeValue, Scope};

#[derive(Debug)]
pub struct AspAndTest {
    not_tests: Vec<AspNotTest>
}

impl AspAndTest {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspAndTest, AspParseError> {
        logger.enter_parser("AspAndTest");

        let a = AspAndTest { not_tests: vec![ AspNotTest::parse(sc, logger)?  ] };

        logger.leave_parser("AspAndTest");
        Ok(a)
    }

    pub fn eval(&self, mut cur_scope: &mut Scope) -> RuntimeValue {
        //TODO
        self.not_tests[0].eval(cur_scope)
    }
}