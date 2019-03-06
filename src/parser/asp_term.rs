use crate::parser::asp_factor::AspFactor;
use crate::parser::asp_term_opr::AspTermOpr;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;

#[derive(Debug)]
pub struct AspTerm {
    factors: Vec<AspFactor>,
    oprs: Vec<AspTermOpr>,
}

impl AspTerm {


    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspTerm,AspParseError> {

        unimplemented!()

    }
}