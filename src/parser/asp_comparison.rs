use crate::parser::asp_comp_opr::AspCompOpr;
use crate::parser::asp_term::AspTerm;

pub struct AspComparison {
    terms: Vec<AspTerm>,
    comp_oprs: Vec<AspCompOpr>
}