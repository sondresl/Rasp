use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::ParseError;


#[derive(Debug)]
pub struct AspAssignment {
    name: AspName,
    expr: AspExpr
}

impl AspAssignment {
    pub fn parse(sc: &mut Scanner) -> Result<AspAssignment,ParseError> {
        match sc.next_token() {
            Token::Name(value) => {
                let name = AspName(value);
                sc.skip(Token::Equal)?;
                let expr = AspExpr::parse(sc)?;
                sc.skip(Token::Newline)?;
                Ok(AspAssignment {name,expr} )
            },
            // TODO: Generalize parse error. Dont hard code Token::Name
            token => Err(ParseError::new(token, Token::Name(String::new()), sc.cur_line()))
        }
    }
}
