use crate::parser::asp_atom::AspAtom;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::parser::asp_arguments::AspArguments;
use crate::parser::asp_syntax::AspSyntax;

#[derive(Debug)]
pub struct AspExpr {
    atom: AspAtom,
    suffix: Option<AspArguments>
}

impl AspSyntax for AspExpr {
    fn parse(sc: &mut Scanner) -> AspExpr {
        let atom = AspAtom::parse(sc);
        let suffix = match sc.cur_token() {
            Token::LeftPar => Some(AspArguments::parse(sc)),
            _ => None
        };
        AspExpr{atom,suffix}
    }
}
