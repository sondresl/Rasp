pub mod scanner;
pub mod token;

#[cfg(test)]
mod scanner_tests {
    use crate::scanner::scanner::Scanner;
    use crate::scanner::token::Token;
    use crate::scanner::token::Token::*;

    fn assert_same_tokens(sc: &mut Scanner, tokens: Vec<Token>) {
        let mut seen_eof = false;
        for token in tokens {
            let t = sc.next_token();
            if t == EoF { seen_eof = true }
            assert_eq!(token, t);
        }
        assert!(seen_eof);
    }

    #[test]
    fn mini_asp() {
        let mut sc = Scanner::new("asp/mini.asp").unwrap();

        let tokens = vec![
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

        assert_same_tokens(&mut sc, tokens);
    }

    #[test]
    fn expressions_asp() {
        let mut sc = Scanner::new("asp/expressions.asp").unwrap();

        let tokens = vec![
            IntegerLiteral(1),
            Plus,
            IntegerLiteral(2),
            Newline,
            IntegerLiteral(2),
            Multiply,
            IntegerLiteral(2),
            Newline,
            EoF
        ];

        assert_same_tokens(&mut sc, tokens);
    }
}