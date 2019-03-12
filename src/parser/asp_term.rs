use crate::scanner::token::Token;

use crate::parser::asp_factor::AspFactor;
use crate::parser::asp_term_opr::AspTermOpr;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::{Scope, RuntimeValue};
use crate::runtime::runtime::RuntimeValue::RuntimeNone;

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

    pub fn eval(&self, mut cur_scope: &mut Scope) -> RuntimeValue {
        //TODO
        let mut a = self.factors[0].eval(cur_scope);
        dbg!(&a);
        for (fac, opr) in self.factors[1..].iter().zip(self.oprs.iter()) {
            dbg!((fac, opr));
            a = match opr {
                AspTermOpr::Plus => a.add(fac.eval(cur_scope)),
                _ => panic!()
            };
        };
        a
    }
}