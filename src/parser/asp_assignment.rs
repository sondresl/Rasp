use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::ParseError;

use std::fs::File;
use std::io::prelude::*;

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

    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspAssignment>\n")?;

        self.name.test_parser(file, indentation + 1)?;
        self.expr.test_parser(file, indentation + 1)?;

        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspAssignment/>\n")?;
        Ok(())
    }
}
