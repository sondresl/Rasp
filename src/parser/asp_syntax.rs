use crate::scanner::scanner::Scanner;

pub trait AspSyntax {

    fn parse(sc: &mut Scanner) -> Self;
}