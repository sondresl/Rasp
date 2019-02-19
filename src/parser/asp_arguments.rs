use crate::parser::asp_expr::AspExpr;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_syntax::AspSyntax;

#[derive(Debug)]
pub struct AspArguments {
    exprs: Vec<AspExpr>
}

impl AspSyntax for AspArguments {
    fn parse(sc: &mut Scanner) -> AspArguments {
        let mut aa = AspArguments{exprs:vec![]};
        sc.skip(Token::LeftPar);
        if let Token::RightPar = sc.cur_token() {
            sc.skip(Token::RightPar);
            return aa;
        }
        loop {
            aa.exprs.push(AspExpr::parse(sc));
            if sc.cur_token() == &Token::Comma {
                sc.skip(Token::Comma);
                continue;
            }
            break;
        }
        sc.skip(Token::RightPar);
        sc.skip(Token::Newline);
        return aa;
    }
}
