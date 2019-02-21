use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token::EoF;
use crate::parser::asp_stmt::AspStmt;
use crate::parser::error::ParseError;


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
}
