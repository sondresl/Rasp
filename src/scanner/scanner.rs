use std::fs::File;
use std::io::{BufReader,BufRead};
use std::io;
use super::token::Token;
use std::result::Result::Ok;
use std::prelude::v1::Vec;
use crate::scanner::token::Token::EoF;

pub struct Scanner {
    token_buffer: Vec<Token>,
    reader: BufReader<File>
}

impl Scanner {

    pub fn new(filename: &str) -> io::Result<Scanner> {
        Ok(Scanner {
            token_buffer: Vec::new(),
            reader: BufReader::new(File::open(filename)?)
        })
    }

    pub fn next_token(&mut self) -> Token {
        while self.token_buffer.is_empty() {
            self.scan_line()
        }

        return self.token_buffer.remove(0);
    }

    fn scan_line(&mut self) {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Err(_) => panic!("Error encountered during scan_line"),
            Ok(0)  => self.token_buffer.push(EoF),
            Ok(_)  => {
                let tokens = scan_line_tokens(line);
                for token in tokens {
                    self.token_buffer.push(token);
                }
            }
        }
    }
}

fn scan_line_tokens(line: String) -> Vec<Token> {

    let mut tokens: Vec<Token> = vec![];

    let trimmed_line = line.trim();
    if trimmed_line.is_empty() || trimmed_line.starts_with("#") { return tokens }

    let vec: Vec<char> = line.chars().collect();
    let mut index = 0;

    while index < vec.len() {
        let c = vec[index];
        // match returns a token, and how much index should increment
        let (token, offset) = match c {
            '='  => (Token::Equal,    1),
            ','  => (Token::Comma,    1),
            '('  => (Token::LeftPar,  1),
            ')'  => (Token::RightPar, 1),
            '\'' => (scan_string(&vec[index+1..])),
            '_'       |
            'a'...'z' |
            'A'...'Z' => scan_name(&vec[index..]),
             _   => { index += 1; continue; }
        };
        tokens.push(token);
        index += offset;
    }
    tokens.push(Token::Newline);
    return tokens;
}

fn scan_name(chars: &[char]) -> (Token, usize) {
    let mut count = 0;
    let mut name = String::new();
    for c in chars.iter() {
        if !(c.is_alphanumeric() || *c == '_') { break; }
        name.push(*c);
        count += 1;
    }
    return (Token::Name(name), count);
}

fn scan_string(chars: &[char]) -> (Token, usize) {
    let mut count = 1;
    let mut string = String::new();
    for c in chars.iter() {
        count += 1;
        if *c == '\'' { break; }
        string.push(*c);
    }
    return (Token::StringLiteral(string), count);
}
