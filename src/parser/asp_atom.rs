use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_string::AspString;
use crate::parser::error::ParseError;


#[derive(Debug)]
pub enum AspAtom {
    Name(AspName),
    String(AspString),
}

impl AspAtom {
    pub fn parse(sc: &mut Scanner) -> Result<AspAtom,ParseError> {
        match sc.next_token() {
            Token::Name(value) => Ok(AspAtom::Name(AspName(value.clone()))),
            Token::StringLiteral(value) => Ok(AspAtom::String(AspString(value.clone()))),
            // TODO: Generalize parse error. Dont hard code Token::Name
            token => Err(ParseError::new(token, Token::Name(String::new()), sc.cur_line()))
        }
    }
}
