use crate::scanner::token::Token;
use std::io;

//#[derive(Debug,new)]
//pub struct ParseError {
//    pub expected: Token,
//    pub found: Token,
//    pub line_number: u32
//}
//
//impl Display for ParseError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "Expected: <{:?}>, but found <{:?}> on line: {}",
//               self.expected,
//               self.found,
//               self.line_number)
//    }
//}

#[derive(Debug,Fail)]
pub enum ParseError {
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

impl From<io::Error> for ParseError {
    fn from(it: io::Error) -> Self {
        ParseError::IO {
            io_error: it
        }
    }
}

