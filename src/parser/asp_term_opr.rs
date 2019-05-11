use crate::scanner::token::Token;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::error::AspParseError;

#[derive(Debug)]
pub enum AspTermOpr {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl AspTermOpr {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspTermOpr, AspParseError> {

        logger.enter_parser("AspTermOpr")?;

        let a = match sc.next_token() {
            Token::Plus => AspTermOpr::Plus,
            Token::Minus => AspTermOpr::Minus,
            Token::Multiply => AspTermOpr::Multiply,
            Token::Divide => AspTermOpr::Divide,
            _            => return Err(AspParseError::IDK), // TODO
        };

        logger.leave_parser("AspTermOpr")?;

        Ok(a)
    }
}