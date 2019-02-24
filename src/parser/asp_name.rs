use crate::log::logger::Logger;
use std::io;
use crate::scanner::scanner::Scanner;
use crate::parser::error::ParseError;
use crate::scanner::token::Token;

#[derive(Debug)]
pub struct AspName(String);

impl AspName {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspName, ParseError> {
        logger.enter_parser("AspName");
        if let Token::Name(name) = sc.next_token() {
            logger.leave_parser("AspName");
            return Ok(AspName(name));
        };
        panic!("Attempted to parse AspName, but no Name token was found")
    }

    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
        logger.enter_parser("AspName")?;
        logger.leave_parser("AspName")
    }
}
