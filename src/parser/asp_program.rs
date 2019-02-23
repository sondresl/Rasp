use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token::EoF;
use crate::parser::asp_stmt::AspStmt;
use crate::parser::error::ParseError;
// use crate::runtime::runtime::RuntimeValue;

use std::fs::File;
use std::io::prelude::*;

use std::fmt;

#[derive(Debug)]
pub struct AspProgram {
    stmts: Vec<AspStmt>
}

impl AspProgram {
    pub fn parse(sc: &mut Scanner) -> Result<AspProgram,ParseError> {
        let mut program = AspProgram{stmts:vec![]};
        while sc.cur_token() != &EoF {
            program.stmts.push(AspStmt::parse(sc)?);
        }
        Ok(program)
    }

    // pub fn eval(&self) -> RuntimeValue {
    //     for v in &self.stmts {
    //         // v.eval();
    //     }
    //     RuntimeValue::RuntimeInteger
    // }

    /// Since this is the method called, it takes no arguments beyond
    /// self, and then sends the writer (and indentation level) down
    /// the stack.
    pub fn test_parser(&self) -> std::io::Result<()> {
        let mut file = File::create("log/mini.log")?;
        file.write_all(b"AspProgram\n")?;
        for s in &self.stmts {
            s.test_parser(&mut file, 1)?;
        }
        Ok(())
    }
}

/// Test of Display trait.
/// Possible to make recursive calls down? To display the whole program tree?
impl fmt::Display for AspProgram {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad("AspProgram")
    }
}
