use std::fs::File;
use std::io::{BufReader,BufRead};
use std::io;
use super::token::Token;
use std::result::Result::Ok;

pub fn scan(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if line.is_empty() || line.starts_with("#") { continue; }
        scan_line(line)
            .expect(&format!("Failed to scan line {} ", i));
    }
    println!("{:?}", Token::EoF);
    Ok(())
}

fn scan_line(line: String) -> Result<(),()> {
    let vec: Vec<char> = line.chars().collect();
    let mut index = 0;
    while index < vec.len() {
        let c = vec[index];
        // match returns a token, and how much index should increment
        let (token, i) = match c {
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
        index += i;
        print!("{:?} ", token);
    }
    println!("{:?}", Token::Newline);
    Ok(())
}

fn scan_name(chars: &[char]) -> (Token, usize) {
    let mut count = 0;
    let mut name = String::new();
    for c in chars.iter() {
        if !(c.is_alphanumeric() || *c == '_') { break; }
        name.push(*c);
        count += 1;
    }
    (Token::Name(name), count)
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
