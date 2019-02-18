use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token;
use crate::scanner::token::Token::EoF;
use crate::parser::parser::AspStmt::ExprStmt;
use crate::parser::parser::AspStmt::Assignment;
use crate::scanner::token::Token::RightPar;
use crate::scanner::token::Token::Newline;

#[derive(Debug)]
pub struct AspProgram {
    stmts: Vec<AspStmt>
}

impl AspProgram {
    pub fn parse(sc: &mut Scanner) -> AspProgram {
        let mut program = AspProgram{stmts:vec![]};
        while sc.cur_token() != &EoF {
            program.stmts.push(AspStmt::parse(sc));
        }
        program
    }
}

#[derive(Debug)]
enum AspStmt {
    Assignment(AspAssignment),
    ExprStmt(AspExpr)
}

impl AspStmt {
    fn parse(sc: &mut Scanner) -> AspStmt {
        if sc.has_equal_token() {
            return Assignment(AspAssignment::parse(sc));
        }
        return ExprStmt(AspExpr::parse(sc));
    }
}

#[derive(Debug)]
struct AspAssignment {
    name: AspName,
    expr: AspExpr
}

impl AspAssignment {
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

#[derive(Debug)]
struct AspExpr {
    atom: AspAtom,
    suffix: Option<AspArguments>
}

impl AspExpr {
    fn parse(sc: &mut Scanner) -> AspExpr {
        let atom = AspAtom::parse(sc);
        let suffix = match sc.cur_token() {
            Token::LeftPar => Some(AspArguments::parse(sc)),
            _ => None
        };
        AspExpr{atom,suffix}
    }
}

#[derive(Debug)]
struct AspName(String);

#[derive(Debug)]
struct AspString(String);

#[derive(Debug)]
enum AspAtom {
    Name(AspName),
    String(AspString)
}

impl AspAtom {
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

#[derive(Debug)]
struct AspArguments {
    exprs: Vec<AspExpr>
}

impl AspArguments {
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
        sc.skip(RightPar);
        sc.skip(Newline);
        return aa;
    }
}