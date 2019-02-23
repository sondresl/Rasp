use crate::parser::asp_atom::AspAtom;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_arguments::AspArguments;
use crate::parser::error::ParseError;
use crate::runtime::runtime::RuntimeValue;
use crate::runtime::runtime::Scope;
use crate::log::logger::Logger;

#[derive(Debug)]
pub struct AspExpr {
    atom: AspAtom,
    suffix: Option<AspArguments>
}

impl AspExpr {
    pub fn parse(sc: &mut Scanner) -> Result<AspExpr,ParseError> {
        let atom = AspAtom::parse(sc)?;
        let suffix = match sc.cur_token() {
            Token::LeftPar => Some(AspArguments::parse(sc)?),
            _ => None
        };
        Ok(AspExpr{atom,suffix})
    }

    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        let rv = RuntimeValue::RuntimeNone;

        rv
    }

    pub fn test_parser(&self, logger: &mut Logger) -> std::io::Result<()> {
        logger.enter_parser("AspExpr")?;
        self.atom.test_parser(logger)?;
        logger.leave_parse("AspExpr")
    }
}
