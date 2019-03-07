use crate::scanner::token::Token;
use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::error::AspParseError;

#[derive(Debug)]
pub enum AspTermOpr {
    Plus,
    Minus
}

impl AspTermOpr {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspTermOpr, AspParseError> {
        Ok(match sc.next_token() {
            Token::Plus => AspTermOpr::Plus,
            Token::Minus => AspTermOpr::Minus,
            _            => return Err(AspParseError::IDK),
        })
    }
}