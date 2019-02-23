#![feature(range_contains)]
#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate derive_new;
extern crate colored;

mod scanner;
mod parser;
mod runtime;
mod log;

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
    println!("Writing to log file...");
    let mut logger = log::logger::Logger::new("log/mini.log").unwrap();
    program.test_parser(&mut logger).expect("Error during test_parser");
    println!("Done");
}

// test_parser is the equivalent of the -testparser flag from INF2100.
// It creates the AST, and then calls pretty print on the tree.
