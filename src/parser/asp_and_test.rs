use crate::parser::asp_not_test::AspNotTest;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;

#[derive(Debug)]
pub struct AspAndTest {
    not_tests: Vec<AspNotTest>
}

impl AspAndTest {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspAndTest, AspParseError> {
        Ok(AspAndTest {
            not_tests: vec![
                AspNotTest::parse(sc, logger)?
            ]
        })
    }
}