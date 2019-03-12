use crate::scanner::scanner::Scanner;
use crate::log::logger::Logger;
use crate::parser::asp_factor_prefix::AspFactorPrefix;
use crate::parser::asp_primary::AspPrimary;
use crate::parser::asp_factor_opr::AspFactorOpr;
use crate::parser::error::AspParseError;


#[derive(Debug, new)]
pub struct AspFactor {
    prefixes: Vec<AspFactorPrefix>,
    primaries: Vec<AspPrimary>,
    oprs: Vec<AspFactorOpr>,
}

impl AspFactor {

    pub fn parse(sc: &mut Scanner, logger: &mut Logger) -> Result<AspFactor,AspParseError> {
        // TODO Prefix and FactorOpr
        logger.enter_parser("AspFactor")?;

        let a = AspFactor::new(vec![], vec![AspPrimary::parse(sc, logger)?], vec![]);

        logger.leave_parser("AspFactor")?;
        Ok(a)
    }
}