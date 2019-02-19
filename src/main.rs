#![feature(range_contains)]
mod scanner;
mod parser;

use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token::EoF;
use crate::parser::asp_program::AspProgram;
use crate::parser::asp_syntax::AspSyntax;

fn main() {
    test_parser()
}

fn test_parser() {
    let mut sc = Scanner::new("asp/mini.asp")
        .expect("Unable to open scanner for asp.mini");

    let program = AspProgram::parse(&mut sc);
    println!("{:?}", program);
}

#[allow(dead_code)]
fn test_scanner() {
    let mut sc = Scanner::new("asp/mini.asp")
        .expect("Unable to open scanner for asp.mini");

    loop {
        let t = sc.next_token();
        println!("{:?}", t);
        if t == EoF { break }
    }
}
