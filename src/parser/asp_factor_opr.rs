use crate::scanner::token::Token;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::error::AspParseError;

#[derive(Debug)]
pub enum AspFactorOpr {
    Multiply,
    Divide,
    IntegerDiv,
}

impl AspFactorOpr {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspFactorOpr, AspParseError> {

        logger.enter_parser("AspTermOpr")?;

        let a = match sc.next_token() {
            Token::Multiply   => AspFactorOpr::Multiply,
            Token::Divide     => AspFactorOpr::Divide,
            Token::IntegerDiv => AspFactorOpr::IntegerDiv,
            _                 => return Err(AspParseError::IDK), // TODO
        };

        logger.leave_parser("AspTermOpr")?;

        Ok(a)
    }
}