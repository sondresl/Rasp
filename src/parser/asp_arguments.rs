use crate::parser::asp_expr::AspExpr;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::error::AspParseError;
use crate::log::logger::Logger;

#[derive(Debug)]
pub struct AspArguments {
    pub exprs: Vec<AspExpr>
}

impl AspArguments {
    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspArguments, AspParseError> {
        logger.enter_parser("AspArguments")?;
        let mut aa = AspArguments{exprs:vec![]};
        sc.skip(Token::LeftPar)?;
        if let Token::RightPar = sc.cur_token() {
            sc.skip(Token::RightPar)?;
            return Ok(aa);
        }
        loop {
            aa.exprs.push(AspExpr::parse(sc, logger)?);
            if sc.cur_token() == &Token::Comma {
                sc.skip(Token::Comma)?;
                continue;
            }
            break;
        }
        sc.skip(Token::RightPar)?;
        sc.skip(Token::Newline)?;
        logger.leave_parser("AspArguments")?;
        Ok(aa)
    }

}
