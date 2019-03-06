use crate::parser::asp_factor_prefix::AspFactorPrefix;
use crate::parser::asp_primary::AspPrimary;
use crate::parser::asp_factor_opr::AspFactorOpr;

#[derive(Debug)]
pub struct AspFactor {
    prefixes: Vec<AspFactorPrefix>,
    primaries: Vec<AspPrimary>,
    oprs: Vec<AspFactorOpr>,
}