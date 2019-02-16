
#[derive(Debug)]
pub enum Token {
    Name(String),
    StringLiteral(String),
    Equal,
    LeftPar,
    RightPar,
    Comma,
    Newline,
    EoF
}