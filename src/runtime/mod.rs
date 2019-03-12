pub mod runtime;

#[cfg(test)]
mod runtime_tests {
    use crate::parser::asp_program::AspProgram;
    use crate::parser::asp_stmt::AspStmt;
    use crate::parser::asp_stmt::AspStmt::ExprStmt;
    use crate::parser::asp_expr::AspExpr;

//    #[test]
//    fn test_exprs() {
//        let tree = AspProgram {
//            stmts: vec![
//                ExprStmt(AspExpr {
//                    atom: (),
//                    suffix: None
//                })
//            ]
//        }
//    }
}
