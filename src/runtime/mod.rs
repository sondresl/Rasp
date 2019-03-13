pub mod runtime;

#[cfg(test)]
mod runtime_tests {
    use crate::parser::asp_program::AspProgram;
    use crate::log::logger::Logger;
    use crate::scanner::scanner::Scanner;
    use std::process::exit;
    use colored::Colorize;
    use crate::parser::error::AspParseError;
    use crate::scanner::token::Token::EoF;
    use crate::parser::asp_expr::AspExpr;
    use crate::runtime::runtime::Scope;
    use crate::scanner::token::Token;

    #[test]
    fn test_exprs() {
        let mut logger = Logger::new("log/expressions.log").unwrap();
        let mut sc = Scanner::new("asp/expressions.asp").unwrap();
        let program = AspProgram::parse(&mut sc, &mut logger).unwrap();
        let x = program.eval();
        dbg!(x);
    }


    #[test]
    fn test_expr_stmt() {
        let mut logger = Logger::new("log/expressions.log").unwrap();
        let mut sc = Scanner::new("asp/expressions.asp").unwrap();
    
        loop {
            let line = sc.fill_token_buffer();
            if line.is_empty() {
                break;
            }
            logger.write(&format!("{:?}", line));
            let a = AspExpr::parse(&mut sc, &mut logger).unwrap();
            sc.skip(Token::Newline);
            logger.write(&format!(" ==> {:?}", a.eval(&mut Scope::new(None))));
        };
    }
}
