use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_string::AspString;
use crate::parser::error::ParseError;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum AspAtom {
    Name(AspName),
    // Is this a legal name ? String? TODO
    StringLit(AspString),
}

impl AspAtom {
    pub fn parse(sc: &mut Scanner) -> Result<AspAtom,ParseError> {
        match sc.next_token() {
            Token::Name(value) => Ok(AspAtom::Name(AspName(value.clone()))),
            Token::StringLiteral(value) => Ok(AspAtom::StringLit(AspString(value.clone()))),
            // TODO: Generalize parse error. Dont hard code Token::Name
            token => Err(ParseError::new(token, Token::Name(String::new()), sc.cur_line()))
        }
    }

    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspAtom>\n")?;

        match self {
            // ??? Why do I need enum name here but not elsewhere?
            AspAtom::Name(v)      => v.test_parser(file, indentation + 1)?,
            AspAtom::StringLit(v) => v.test_parser(file, indentation + 1)?,
        };

        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspAtom/>\n")?;
        Ok(())
    }
}
