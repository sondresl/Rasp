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


        logger.enter_parser("AspTerm")?;

//        let a = AspTerm::new(vec![AspFactor::parse(sc, logger)?], vec![]);

        let mut factors = vec![];
        let mut oprs = vec![];

        loop {
            // TODO: Prefix
            factors.push(AspFactor::parse(sc, logger)?);
            match sc.cur_token() {
                &Token::Plus  |
                &Token::Minus => oprs.push(AspTermOpr::parse(sc, logger)?),
                _             => break
            };
        }

        logger.leave_parser("AspTerm")?;

        Ok(AspTerm::new(factors, oprs))
    }
}