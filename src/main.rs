#![feature(range_contains)]

#[macro_use]
extern crate derive_new;
extern crate colored;

mod scanner;
mod parser;
mod runtime;

use crate::scanner::scanner::Scanner;
use crate::parser::asp_program::AspProgram;
use std::process::exit;
use colored::Colorize;

fn main() {
    println!("This is the Rasp Interpreter");
    let mut sc = Scanner::new("asp/mini.asp").unwrap();
    let program = AspProgram::parse(&mut sc).unwrap_or_else(|p| {
        println!("{}", "Error! Failed to parse program:".red());
        println!("{}", p);
        exit(1);
    });
    // program.eval();
    program.test_parser();
    // println!("{:?}", program);
}

// test_parser is the equivalent of the -testparser flag from INF2100.
// It creates the AST, and then calls pretty print on the tree.
