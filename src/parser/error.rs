use crate::scanner::token::Token;
use std::io;

#[derive(Debug,Fail)]
pub enum AspParseError {
    #[fail(display = "Expected <{:#?}>, but found <{:#?}> on line {}", expected, found, line_number)]
    Expected {
        expected: Token,
        found: Token,
        line_number: usize
    },
    #[fail(display = "Expected one of <{:#?}>, but found <{:#?}> on line {}", expected, found, line_number)]
    ExpectedOneOf {
        expected: Vec<Token>,
        found: Token,
        line_number: usize
    },
    #[fail(display = "Encountered IO Error during parsing: {}", io_error)]
    IO {
        io_error: io::Error
    }
}

impl From<io::Error> for AspParseError {
    fn from(it: io::Error) -> Self {
        AspParseError::IO {
            io_error: it
        }
    }
}

