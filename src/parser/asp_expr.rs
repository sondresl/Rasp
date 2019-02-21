use crate::parser::asp_atom::AspAtom;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_arguments::AspArguments;
use crate::parser::error::ParseError;


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
}
