use crate::scanner::token::Token;

use crate::parser::asp_factor::AspFactor;
use crate::parser::asp_term_opr::AspTermOpr;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;

#[derive(Debug, new)]
pub struct AspTerm {
    factors: Vec<AspFactor>,
    oprs: Vec<AspTermOpr>,
}

impl AspTerm {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspTerm,AspParseError> {
        // let mut factors = vec![];
        // let mut oprs = vec![];
        // factors.push(AspFactor::parse(sc, logger)?);
        // match sc.cur_token() {
        //     &Token::Plus | &Token::Minus 
        //         => AspTermOpr::parse(sc, logger)?,
        //     _   => return Err(AspParseError::IDK)
        // };
        // Ok(AspTerm::new(factors, oprs))

        logger.enter_parser("AspTerm")?;

        let a = AspTerm::new(vec![AspFactor::parse(sc, logger)?], vec![]);

        logger.leave_parser("AspTerm")?;
        Ok(a)
    }
}