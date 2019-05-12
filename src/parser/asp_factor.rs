use crate::scanner::token::Token;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_factor_prefix::AspFactorPrefix;
use crate::parser::asp_primary::AspPrimary;
use crate::parser::asp_factor_opr::AspFactorOpr;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::{Scope, RuntimeValue};


#[derive(Debug, new)]
pub struct AspFactor {
    prefixes: Vec<AspFactorPrefix>,
    primaries: Vec<AspPrimary>,
    oprs: Vec<AspFactorOpr>,
}

impl AspFactor {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspFactor,AspParseError> {
        // TODO Prefix and FactorOpr
        logger.enter_parser("AspFactor")?;

        let mut primaries = vec![];
        let mut prefixes = vec![];
        let mut oprs = vec![];

        loop {
            // TODO prefixes
            primaries.push(AspPrimary::parse(sc, logger)?);
            match sc.cur_token() {
                &Token::Multiply  |
                &Token::Divide    |
                &Token::IntegerDiv => oprs.push(AspFactorOpr::parse(sc, logger)?),
                _                  => break
            };
        }

        logger.leave_parser("AspFactor")?;
        Ok(AspFactor::new(prefixes, primaries, oprs))
    }

    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        self.primaries[1..].iter()
            .zip(self.oprs.iter())
            .fold(self.primaries[0].eval(cur_scope), |rv, (primary, opr)| {
                match opr {
                    AspFactorOpr::Multiply   => rv * primary.eval(cur_scope),
                    AspFactorOpr::Divide     => rv / primary.eval(cur_scope),
                    AspFactorOpr::IntegerDiv => rv.int_div(primary.eval(cur_scope)),
                }
            })
    }
}