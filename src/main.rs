#![feature(range_contains)]
mod scanner;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::Token::EoF;

fn main() {

    let mut sc = Scanner::new("asp/mini.asp")
        .expect("Unable to open scanner for asp.mini");

    loop {
        let t = sc.next_token();
        println!("{:?}", t);
        if t == EoF { break }
    }
}
