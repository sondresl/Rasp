use crate::parser::asp_atom::AspAtom;
use crate::parser::asp_primary_suffix::AspPrimarySuffix;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_arguments::AspArguments;
use crate::scanner::token::Token;
use crate::runtime::runtime::RuntimeValue;
use crate::parser::asp_expr::AspExpr;
use crate::runtime::runtime::Scope;
use std::io;
use crate::parser::error::AspParseError;

#[derive(Debug)]
pub struct AspPrimary {
    atom: AspAtom,
    suffix: Option<AspPrimarySuffix>,
}

impl AspPrimary {
    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspPrimary, AspParseError> {

        logger.enter_parser("AspPrimary")?;

        let atom = AspAtom::parse(sc, logger)?;
        let suffix = match sc.cur_token() {
            Token::LeftPar => Some(AspPrimarySuffix::Argument(AspArguments::parse(sc, logger)?)),
            _ => None
        };

        logger.leave_parser("AspPrimary")?;

        Ok(AspPrimary{atom,suffix})
    }

    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        let rv = RuntimeValue::RuntimeNone;

        rv
    }

}
