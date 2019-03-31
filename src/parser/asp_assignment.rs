use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_expr::AspExpr;
use crate::parser::error::AspParseError;
use crate::runtime::runtime::Scope;
use crate::log::logger::Logger;

#[derive(Debug)]
pub struct AspAssignment {
    pub name: AspName,
    pub expr: AspExpr
}

impl AspAssignment {
    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspAssignment, AspParseError> {

        logger.enter_parser("AspAssignment")?;

        let asp_assignment = match sc.cur_token() {
            Token::Name(_) => {
                let name = AspName::parse(sc, logger)?;
                sc.skip(Token::Equal)?;
                let expr = AspExpr::parse(sc, logger)?;
                sc.skip(Token::Newline)?;
                Ok(AspAssignment {name,expr} )
            },
            token => return Err(AspParseError::Expected {
                expected: Token::Name(String::new()),
                found: token.clone(),
                line_number: sc.cur_line() as usize
            })
        };

        logger.leave_parser("AspAssignment")?;
        asp_assignment
    }

    pub fn eval(&self, cur_scope: &mut Scope) {
        let v = self.expr.eval(cur_scope);
        cur_scope.insert(self.name.0.clone(), v);
    }

}
