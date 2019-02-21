pub mod asp_arguments;
pub mod asp_assignment;
pub mod asp_atom;
pub mod asp_expr;
pub mod asp_name;
pub mod asp_program;
pub mod asp_stmt;
pub mod asp_string;
pub mod error;

#[cfg(test)]
mod parser_tests {
    use crate::scanner::scanner::Scanner;
    use crate::parser::asp_program::AspProgram;

    use std::io::BufReader;
    use std::collections::VecDeque;

    #[test]
    fn mini_asp() {
        let mut sc = Scanner::new("asp/mini.asp").unwrap();
        let program = AspProgram::parse(&mut sc);
        println!("{:?}", program);
    }
}