pub mod scanner;
pub mod token;

#[cfg(test)]
mod scanner_tests {
    use crate::scanner::scanner::Scanner;
    use crate::scanner::token::Token;
    use crate::scanner::token::Token::*;

    #[test]
    fn mini_asp() {

        let mut sc = Scanner::new("asp/mini.asp").unwrap();

        let tokens: Vec<Token> = vec![
            Name(String::from("navn")),
            Equal,
            StringLiteral(String::from("Dag")),
            Newline,
            Name(String::from("print")),
            LeftPar,
            StringLiteral(String::from("Hei,")),
            Comma,
            Name(String::from("navn")),
            RightPar,
            Newline,
            EoF
        ];

        let mut seen_eof = false;
        for token in tokens {
            let t = sc.next_token();
            if t == EoF { seen_eof = true }
            assert_eq!(token, t);
        }
        assert!(seen_eof);
    }
}