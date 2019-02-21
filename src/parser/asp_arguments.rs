use crate::parser::asp_expr::AspExpr;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::error::ParseError;

#[derive(Debug)]
pub struct AspArguments {
    exprs: Vec<AspExpr>
}

impl AspArguments {
    pub fn parse(sc: &mut Scanner) -> Result<AspArguments,ParseError> {
        let mut aa = AspArguments{exprs:vec![]};
        sc.skip(Token::LeftPar)?;
        if let Token::RightPar = sc.cur_token() {
            sc.skip(Token::RightPar)?;
            return Ok(aa);
        }
        loop {
            aa.exprs.push(AspExpr::parse(sc)?);
            if sc.cur_token() == &Token::Comma {
                sc.skip(Token::Comma)?;
                continue;
            }
            break;
        }
        sc.skip(Token::RightPar)?;
        sc.skip(Token::Newline)?;
        Ok(aa)
    }
}
