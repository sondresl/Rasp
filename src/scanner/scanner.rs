use std::fs::File;
use std::io::{BufReader,BufRead};
use std::io;
use super::token::Token;
use std::str::Chars;

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
//    let mut buf: Vec<Char> = vec![];
    let mut char_iter: Chars = line.chars();
    let mut c = char_iter.next();
    dbg!(line);
    while c.is_some() {
        let token = match c.unwrap() {
            '='  => Some(Token::Equal),
            ','  => Some(Token::Comma),
            '('  => Some(Token::LeftPar),
            ')'  => Some(Token::RightPar),
            '\'' => Some(Token::StringLiteral(scan_string(c.unwrap(),&mut char_iter))),
            '_'       |
            'a'...'z' |
            'A'...'Z' => Some(Token::Name(scan_name(c.unwrap(),&mut char_iter))),
             _   => None
        };
        c = char_iter.next();
        if token.is_none() { continue }
        println!("{:?}", token.unwrap());
    }
    println!("{:?}", Token::Newline);
    Ok(())
}

fn scan_name(c: char, cs: &mut Chars) -> String {
    let mut name = c.to_string();
    let mut c = cs.next();
    while c.is_some() {
        if !(c.unwrap().is_alphanumeric() || c.unwrap() == '_') { break }
        name.push(c.unwrap());
        c = cs.next();
    }
    cs.next_back();
    name
}

fn scan_string(c: char, cs: &mut Chars) -> String {
    let mut name = c.to_string();
    let mut c = cs.next();
    while c.is_some() {
        name.push(c.unwrap());
        if c.unwrap() == '\'' { break }
        c = cs.next();
    }
    return name;
}
