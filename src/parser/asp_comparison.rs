use crate::parser::asp_comp_opr::AspCompOpr;
use crate::parser::asp_term::AspTerm;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::{Scope, RuntimeValue};

#[derive(Debug)]
pub struct AspComparison {
    terms: Vec<AspTerm>,
    comp_oprs: Vec<AspCompOpr>,
}

impl AspComparison {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspComparison, AspParseError> {

        logger.enter_parser("AspComparison")?;

        // TODO
        let a = AspComparison { terms: vec![ AspTerm::parse(sc, logger)?  ], comp_oprs: vec![] };

        logger.leave_parser("AspComparison")?;
        Ok(a)
    }

    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        //TODO
        self.terms[0].eval(cur_scope)
    }
}