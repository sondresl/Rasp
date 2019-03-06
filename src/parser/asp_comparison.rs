use crate::parser::asp_comp_opr::AspCompOpr;
use crate::parser::asp_term::AspTerm;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;

#[derive(Debug)]
pub struct AspComparison {
    terms: Vec<AspTerm>,
    comp_oprs: Vec<AspCompOpr>,
}

impl AspComparison {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspComparison,AspParseError> {
        Ok(AspComparison {
            terms: vec![
                AspTerm::parse(sc, logger)?
            ],
            comp_oprs: vec![]
        })
    }
}