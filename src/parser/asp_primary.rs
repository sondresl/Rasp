use crate::parser::asp_atom::AspAtom;
use crate::parser::asp_primary_suffix::AspPrimarySuffix;

pub struct AspPrimary {
    atom: AspAtom,
    suffixes: Vec<AspPrimarySuffix>
}


