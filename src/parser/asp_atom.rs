use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_name::AspName;
use crate::parser::asp_string::AspString;
use crate::parser::asp_syntax::AspSyntax;

#[derive(Debug)]
pub enum AspAtom {
    Name(AspName),
    String(AspString),
}

impl AspSyntax for AspAtom {
    fn parse(sc: &mut Scanner) -> AspAtom {
        let atom = match sc.cur_token() {
            Token::Name(value) => AspAtom::Name(AspName(value.clone())),
            Token::StringLiteral(value) => AspAtom::String(AspString(value.clone())),
            _ => panic!()
        };
        sc.next_token();
        atom
    }
}
