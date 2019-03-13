use super::token::Token;
use crate::scanner::token::Token::EoF;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::prelude::v1::Vec;
use std::result::Result::Ok;
use crate::parser::error::AspParseError;

pub struct Scanner {
    token_buffer: VecDeque<Token>,
    reader: BufReader<File>,
    cur_line: u32
}

impl Scanner {
    pub fn new(filename: &str) -> io::Result<Scanner> {
        Ok(Scanner {
            token_buffer: VecDeque::new(),
            reader: BufReader::new(File::open(filename)?),
            cur_line: 0
        })
    }

    pub fn cur_line(&self) -> u32 {
        self.cur_line
    }

    pub fn cur_token(&mut self) -> &Token {
        self.fill_token_buffer();
        self.token_buffer.front().unwrap()
    }


    pub fn next_token(&mut self) -> Token {
        self.fill_token_buffer();
        return self.token_buffer.pop_front().unwrap();
    }

    pub fn skip(&mut self, token: Token) -> Result<(), AspParseError> {
        let t = self.next_token();
        if t == token { return Ok(()) }
        return Err(AspParseError::Expected{expected:token, found:t, line_number:self.cur_line as usize})
    }

    pub fn has_equal_token(&self) -> bool {
        self.token_buffer.contains(&Token::Equal)
    }

    fn scan_line(&mut self, s: &mut String) {
        self.cur_line += 1;
        let mut line = String::new();

        let it = self.reader.read_line(&mut line);
        s.push_str(&line[..]);

        match it {
            Err(e) => panic!(format!("Error encountered during scan_line:\n{:?}", e)),
            Ok(0)  => self.token_buffer.push_back(EoF),
            Ok(_)  => scan_line_tokens(line, &mut self.token_buffer),
        }
    }

    pub fn fill_token_buffer(&mut self) -> String {
        let mut s = String::new();
        while self.token_buffer.is_empty() {
            self.scan_line(&mut s)
        }
        s
    }

}

/// Scan one line of input and create tokens.
fn scan_line_tokens(line: String, tokens: &mut VecDeque<Token>) {
    let trimmed_line = line.trim();
    if trimmed_line.is_empty() || trimmed_line.starts_with("#") {
        return;
    }

    let chars: Vec<char> = line.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        let c = chars[index];
        // match returns a token, and how much index should increment
        let (token, offset) = match c {
            '='       => (Token::Equal,     1),
            ','       => (Token::Comma,     1),
            '('       => (Token::LeftPar,   1),
            ')'       => (Token::RightPar,  1),
            '+'       => (Token::Plus,      1),
            '\''      => scan_string(&chars[index + 1..]),
            '1'...'9' => scan_number(&chars[index..]),
            '_'       |
            'a'...'z' |
            'A'...'Z' => scan_name(&chars[index..]),
             _        => { index += 1; continue; }
        };
        tokens.push_back(token);
        index += offset;
    }
    tokens.push_back(Token::Newline);
}

/// Scans a name token.
/// Iterates through the string until a non-name character is found.
fn scan_name(chars: &[char]) -> (Token, usize) {
    let mut offset = 0;
    let mut name = String::new();
    for c in chars.iter() {
        if !(c.is_alphanumeric() || *c == '_') {
            break;
        }
        name.push(*c);
        offset += 1;
    }
    (Token::Name(name), offset)
}

/// Scans until the next '.
/// Currently only implemented for single quotes.
fn scan_string(chars: &[char]) -> (Token, usize) {
    let mut offset = 1;
    let mut string = String::new();
    for c in chars.iter() {
        offset += 1;
        if *c == '\'' {
            break;
        }
        string.push(*c);
    }
    (Token::StringLiteral(string), offset)
}

fn scan_number(chars: &[char]) -> (Token, usize) {
    let mut offset = 0;
    let mut number = String::new();
    for c in chars.iter() {
        if !c.is_numeric() {
            break;
        }
        number.push(*c);
        offset += 1;
    }
    (Token::IntegerLiteral(number.parse().unwrap()), offset)
}
