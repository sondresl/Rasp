use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token::EoF;
use crate::parser::asp_stmt::AspStmt;
use crate::parser::error::ParseError;
use crate::runtime::runtime::RuntimeValue;
use crate::runtime::runtime::Scope;
use crate::log::logger::Logger;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct AspProgram {
    stmts: Vec<AspStmt>
}

impl AspProgram {
    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspProgram,ParseError> {

        logger.enter_parser("AspProgram");

        let mut program = AspProgram{stmts:vec![]};
        while sc.cur_token() != &EoF {
            program.stmts.push(AspStmt::parse(sc, logger)?);
        }

        logger.leave_parser("AspProgram");

        Ok(program)
    }

    /// As the highest level in the ast, eval is first called on this instance,
    /// which calls it on all the elements in the stmts vec. 
    ///
    /// This function should have a global scope, with builtin functions.
    ///
    pub fn eval(&self) -> RuntimeValue {
        let mut sc = Scope::new(None);
        let rv = RuntimeValue::RuntimeNone;
        for v in &self.stmts {
            v.eval(&mut sc);
        }
        rv
    }

    /// Since this is the method called, it takes no arguments beyond
    /// self, and then sends the writer (and indentation level) down
    /// the stack.
    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
        logger.enter_parser("AspProgram")?;
        for s in &self.stmts {
            s.test_parser(logger)?;
        }
        logger.leave_parser("AspProgram")
    }
}

/// Test of Display trait.
/// Possible to make recursive calls down? To display the whole program tree?
impl fmt::Display for AspProgram {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad("AspProgram")
    }
}
