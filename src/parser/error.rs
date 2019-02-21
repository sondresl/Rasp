use std::fmt::Display;
use std::fmt;
use crate::scanner::token::Token;

#[derive(Debug,new)]
pub struct ParseError {
    pub expected: Token,
    pub found: Token,
    pub line_number: u32
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected: <{:?}>, but found <{:?}> on line: {}",
               self.expected,
               self.found,
               self.line_number)
    }
}
