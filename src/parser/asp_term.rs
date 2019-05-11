use crate::scanner::token::Token;

use crate::parser::asp_factor::AspFactor;
use crate::parser::asp_term_opr::AspTermOpr;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::{Scope, RuntimeValue};

#[derive(Debug, new)]
pub struct AspTerm {
    factors: Vec<AspFactor>,
    oprs: Vec<AspTermOpr>,
}

impl AspTerm {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspTerm,AspParseError> {

        logger.enter_parser("AspTerm")?;

        let mut factors = vec![];
        let mut oprs = vec![];

        loop {
            // TODO: Prefix
            factors.push(AspFactor::parse(sc, logger)?);
            match sc.cur_token() {
                &Token::Plus      |
                &Token::Minus     |
                &Token::Multiply  |
                &Token::Divide    => oprs.push(AspTermOpr::parse(sc, logger)?),
                _                 => break
            };
        }

        logger.leave_parser("AspTerm")?;

        Ok(AspTerm::new(factors, oprs))
    }

    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        self.factors[1..].iter()
            .zip(self.oprs.iter())
            .fold(self.factors[0].eval(cur_scope), |rv, (factor, opr)| {
                match opr {
                    AspTermOpr::Plus     => rv.add(factor.eval(cur_scope)),
                    AspTermOpr::Minus    => rv.minus(factor.eval(cur_scope)),
                    AspTermOpr::Multiply => rv.multiply(factor.eval(cur_scope)),
                    AspTermOpr::Divide   => rv.divide(factor.eval(cur_scope)),
                }
            })
    }
}