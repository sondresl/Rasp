use crate::log::logger::Logger;
use crate::scanner::scanner::Scanner;
use crate::parser::error::AspParseError;
use crate::scanner::token::Token;

#[derive(Debug)]
pub struct AspName(pub String);

impl AspName {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspName, AspParseError> {

        logger.enter_parser("AspName")?;

        if let Token::Name(name) = sc.next_token() {
            logger.leave_parser("AspName")?;
            return Ok(AspName(name));
        };

        panic!("Attempted to parse AspName, but no Name token was found")
    }

}
