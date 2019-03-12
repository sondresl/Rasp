pub mod runtime;

#[cfg(test)]
mod runtime_tests {
    use crate::parser::asp_program::AspProgram;
    use crate::log::logger::Logger;
    use crate::scanner::scanner::Scanner;

    #[test]
    fn test_exprs() {
        let mut logger = Logger::new("log/expressions.log").unwrap();
        let mut sc = Scanner::new("asp/expressions.asp").unwrap();
        let program = AspProgram::parse(&mut sc, &mut logger).unwrap();
        let x = program.eval();
        dbg!(x);
    }
}
