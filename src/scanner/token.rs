
#[derive(Debug,PartialEq, Clone)]
pub enum Token {
    Name(String),
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Equal,
    LeftPar,
    RightPar,
    Comma,
    Newline,
    Plus,
    Minus,
    Multiply,
    Divide,
    IntegerDiv,
    EoF
}