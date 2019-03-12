use crate::log::logger::Logger;
use crate::scanner::scanner::Scanner;
use crate::parser::error::AspParseError;
use crate::scanner::token::Token;

#[derive(Debug)]
pub struct AspString(String);

impl AspString {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspString, AspParseError> {
        logger.enter_parser("AspString")?;
        if let Token::StringLiteral(string) = sc.next_token() {
            logger.leave_parser("AspString")?;
            return Ok(AspString(string));
        };
        panic!("Attempted to parse AspString, but no StringLiteral was found")
    }

//    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
//        logger.enter_parser("AspString")?;
//        logger.leave_parser("AspString")
//    }
}
