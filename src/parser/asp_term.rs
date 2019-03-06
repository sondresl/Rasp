use crate::parser::asp_factor::AspFactor;
use crate::parser::asp_term_opr::AspTermOpr;

pub struct AspTerm {
    factors: Vec<AspFactor>,
    oprs: Vec<AspTermOpr>
}