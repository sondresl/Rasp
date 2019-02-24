use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_string::AspString;
use crate::parser::error::ParseError;
use crate::log::logger::Logger;
use std::io;

#[derive(Debug)]
pub enum AspAtom {
    Name(AspName),
    // Is this a legal name ? String? TODO
    StringLit(AspString),
}

impl AspAtom {
    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspAtom,ParseError> {

        logger.enter_parser("AspAtom");

        let asp_atom = match sc.cur_token() {
            Token::Name(value) => AspAtom::Name(AspName::parse(sc, logger)?),
            Token::StringLiteral(value) => AspAtom::StringLit(AspString::parse(sc, logger)?),
            // TODO: Generalize parse error. Dont hard code Token::Name
//            token => Err(ParseError::new(token, Token::Name(String::new()), sc.cur_line()))
            token => panic!("")
        };

        logger.leave_parser("AspAtom");
        Ok(asp_atom)
    }

    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
        logger.enter_parser("AspAtom")?;
        match self {
            // ??? Why do I need enum name here but not elsewhere?
            AspAtom::Name(v)      => v.test_parser(logger)?,
            AspAtom::StringLit(v) => v.test_parser(logger)?,
        };

        logger.leave_parser("AspAtom")
    }
}
