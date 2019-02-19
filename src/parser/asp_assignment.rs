use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_expr::AspExpr;
use crate::parser::asp_syntax::AspSyntax;

#[derive(Debug)]
pub struct AspAssignment {
    name: AspName,
    expr: AspExpr
}

impl AspSyntax for AspAssignment {
    fn parse(sc: &mut Scanner) -> AspAssignment {
        if let Token::Name(value) = sc.next_token() {
            let name = AspName(value);
            sc.skip(Token::Equal);
            let expr = AspExpr::parse(sc);
            sc.skip(Token::Newline);
            return AspAssignment {name,expr};
        }
        panic!()
    }
}
