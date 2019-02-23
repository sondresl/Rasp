use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token::EoF;
use crate::parser::asp_stmt::AspStmt;
use crate::parser::error::ParseError;
use crate::runtime::runtime::RuntimeValue;
use crate::runtime::runtime::Scope;

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
    pub fn test_parser(&self) -> std::io::Result<()> {
        let mut file = File::create("log/mini.log")?;
        file.write(b"<AspProgram>\n")?;
        for s in &self.stmts {
            s.test_parser(&mut file, 1)?;
        }

        file.write(b"<AspProgram/>\n")?;
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
