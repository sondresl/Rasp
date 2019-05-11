#![allow(dead_code, unused_variables)]

extern crate colored;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate derive_new;

mod scanner;
mod parser;
mod runtime;
mod log;

use crate::scanner::scanner::Scanner;
use crate::parser::asp_program::AspProgram;
use crate::log::logger::Logger;
use std::process::exit;
use colored::Colorize;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() < 2 {
        "mini.asp".to_string()
    } else {
        args[1].to_string()
    };

    let infile   = format!("asp/{}", filename);
    let logfile  = format!("log/{}.log", &filename[..4]);

    // println!("This is the Rasp Interpreter\n");

    // println!("Writing to log file '{}' ... ", logfile);
    let mut logger = Logger::new(&logfile).unwrap();

    // println!("Parsing {}", filename);
    let mut sc = Scanner::new(&infile).unwrap();


    let program = AspProgram::parse(&mut sc, &mut logger).unwrap_or_else(|p| {
        println!("{}", "Error! Failed to parse program:".red());
        println!("{}", p);
        exit(1);
    });
    program.eval();
//    program.test_parser(&mut logger).expect("Error during test_parser");
    // println!("Done");
}


// test_parser is the equivalent of the -testparser flag from INF2100.
// It creates the AST, and then calls pretty print on the tree.
