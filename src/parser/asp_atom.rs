use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_string::AspString;
use crate::parser::error::ParseError;
use crate::log::logger::Logger;

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

    pub fn test_parser(&self, logger: &mut Logger) -> std::io::Result<()> {
        logger.enter_parser("AspAtom")?;
        match self {
            // ??? Why do I need enum name here but not elsewhere?
            AspAtom::Name(v)      => v.test_parser(logger)?,
            AspAtom::StringLit(v) => v.test_parser(logger)?,
        };

        logger.leave_parse("AspAtom")
    }
}
