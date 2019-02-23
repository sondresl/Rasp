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
        file.write_all(b"<AspAssignment>\n");
        match self {
            // ???
            AspAtom::Name(v)   => file.write_all(b"  <AspName>\n"),
            AspAtom::StringLit(v) => file.write_all(b"  <AspString>\n"),
        };
        file.write_all(b"<\\AspAssignment\\>\n");
        Ok(())
    }
}
