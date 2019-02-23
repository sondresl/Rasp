use crate::parser::asp_atom::AspAtom;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_arguments::AspArguments;
use crate::parser::error::ParseError;

use std::fs::File;
use std::io::prelude::*;

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

    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspExpr>\n")?;

        self.atom.test_parser(file, indentation + 1)?;

        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspExpr/>\n")?;
        Ok(())
    }
}
