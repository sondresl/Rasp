
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::io;
use super::token::Token;
use std::str::Chars;

pub fn scan(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        //if line.is_empty() || line.starts_with("#") { continue; }
        scan_line(line?.as_str());
    }
    Ok(())
}

fn scan_line(line: &str) -> Result<(),()> {
//    let mut buf: Vec<Char> = vec![];
    let mut char_iter: Chars = line.chars();
    let mut c = char_iter.next();
    while c.is_some() {
        let token = match c.unwrap() {
            '=' => Some(Token::Equal),
            ',' => Some(Token::Comma),
            '_'       |
            'a'...'z' |
            'A'...'Z' => Some(Token::Name(scan_name(c.unwrap(),char_iter))),
            _  => None,
        };
        c = char_iter.next();
        if token.is_none() { continue }
        println!("{:?}", token.unwrap());
    }
    Ok(())
}

fn scan_name(c: char, cs: Chars) -> String {
    let mut name = String::from(c);
    let x = cs.take_while(|it|
        it == '_' ||
        ('a'..='z').contains(&c) ||
        ('A'..='Z').contains(&c))
        .collect();
    name.push_str(x);
    name
}