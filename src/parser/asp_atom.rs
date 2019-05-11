use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_string::AspString;
use crate::parser::error::AspParseError;
use crate::log::logger::Logger;
use crate::parser::asp_literals::{AspInteger, AspFloat};
use crate::runtime::runtime::{Scope, RuntimeValue};
use crate::runtime::runtime::RuntimeValue::{RuntimeInteger, RuntimeFloat};
use crate::runtime::runtime::RuntimeValue::RuntimeString;

#[derive(Debug)]
pub enum AspAtom {
    Name(AspName),
    String(AspString),
    Int(AspInteger),
    Float(AspFloat),
}

impl AspAtom {
    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspAtom, AspParseError> {

        logger.enter_parser("AspAtom")?;

        let asp_atom = match sc.cur_token() {
            Token::Name(value) => AspAtom::Name(AspName::parse(sc, logger)?),
            Token::StringLiteral(value) => AspAtom::String(AspString::parse(sc, logger)?),
            Token::IntegerLiteral(value) => AspAtom::Int(AspInteger::parse(sc, logger)?),
            Token::FloatLiteral(value)   => AspAtom::Float(AspFloat::parse(sc, logger)?),
            token => return Err(AspParseError::ExpectedOneOf {
                expected: vec![Token::Name(String::new()), 
                               Token::StringLiteral(String::new()), 
                               Token::IntegerLiteral(0), 
                               Token::FloatLiteral(0.0)],
                found: token.clone(),
                line_number: sc.cur_line() as usize

            })
        };

        logger.leave_parser("AspAtom")?;
        Ok(asp_atom)
    }

    pub fn eval(&self, cur_scope: &mut Scope) -> RuntimeValue {
        match self {
            AspAtom::Int(v)    => RuntimeInteger(v.0),
            AspAtom::Float(v)    => RuntimeFloat(v.0),
            AspAtom::String(v) => RuntimeString(v.0.clone()),
            AspAtom::Name(v)   => {
                let a = cur_scope.find(v.0.clone());
                let a = a.expect("Asp atom");
                (*a).clone()
            }
        }
    }
}
