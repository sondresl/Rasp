use std::fs::File;
use std::io::{BufReader,BufRead};
use std::io;
use super::token::Token;
use std::str::Chars;
use std::result::Result::Ok;

pub fn scan(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line?.clone();
        if line.is_empty() || line.starts_with("#") { continue; }
        scan_line(line.as_str());
    }
    Ok(())
}

fn scan_line(line: &str) -> Result<(),()> {
    let vec: Vec<char> = line.chars().collect();
    let mut index = 0;
    while index < vec.len() {
        let c = vec[index];
        let (token, i) = match c {
            '='  => (Token::Equal,  index+1),
            ','  => (Token::Comma,  index+1),
            '('  => (Token::LeftPar,index+1),
            ')'  => (Token::RightPar,index+1),
            '\'' => (scan_string(&vec, index)),
            '_'       |
            'a'...'z' |
            'A'...'Z' => scan_name(&vec, index),
             _   => { index += 1;continue; }
        };
        index = i;
        println!("{:?}", token);
    }
    println!("{:?}", Token::Newline);
    Ok(())
}

fn scan_name(vec: &[char], index: usize) -> (Token, usize) {
    let mut count = index;
    let mut name = String::new();
    for c in vec[index..].iter() {
        if !(c.is_alphanumeric() || *c == '_') { break; }
        name.push(*c);
        count += 1;
    }
    (Token::Name(name), count)
}

fn scan_string(vec: &[char], index: usize) -> (Token,usize) {
    let mut count = index+1;
    let mut string = String::new();
    for c in vec[(index+1)..].iter() {
        count += 1;
        if *c == '\'' { break; }
        string.push(*c);
    }
    return (Token::StringLiteral(string), count);
}
