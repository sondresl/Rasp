pub mod asp_arguments;
pub mod asp_assignment;
pub mod asp_atom;
pub mod asp_expr;
pub mod asp_name;
pub mod asp_program;
pub mod asp_stmt;
pub mod asp_string;
pub mod error;
pub mod asp_and_test;
pub mod asp_not_test;
pub mod asp_comparison;
pub mod asp_term;
pub mod asp_comp_opr;
pub mod asp_term_opr;
pub mod asp_factor;
pub mod asp_factor_opr;
pub mod asp_factor_prefix;
pub mod asp_primary;
pub mod asp_primary_suffix;
pub mod asp_subscription;
pub mod asp_integer;

#[cfg(test)]
mod parser_tests {
    use crate::scanner::scanner::Scanner;
    use crate::parser::asp_program::AspProgram;

    use std::io::BufReader;
    use std::collections::VecDeque;
    use crate::log::logger::Logger;

    #[test]
    fn mini_asp() {
        let mut logger = Logger::new("log/mini.log").unwrap();
        let mut sc = Scanner::new("asp/mini.asp").unwrap();
        let program = AspProgram::parse(&mut sc, &mut logger).unwrap();
        println!("{:?}", program);
    }

    #[test]
    fn expressions_asp() {
        let mut logger = Logger::new("log/expressions.log").unwrap();
        let mut sc = Scanner::new("asp/expressions.asp").unwrap();
        let program = AspProgram::parse(&mut sc, &mut logger).unwrap();
        println!("{:?}", program);
    }

}