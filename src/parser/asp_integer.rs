use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::error::AspParseError;
use crate::scanner::token::Token;

#[derive(Debug)]
pub struct AspInteger(i64);

impl AspInteger {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspInteger, AspParseError> {
        logger.enter_parser("AspInteger")?;
        if let Token::IntegerLiteral(integer) = sc.next_token() {
            logger.leave_parser("AspInteger")?;
            return Ok(AspInteger(integer));
        };
        panic!("Attempted to parse AspString, but no StringLiteral was found")
    }
}