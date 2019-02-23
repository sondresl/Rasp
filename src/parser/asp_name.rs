use crate::log::logger::Logger;
use std::io;

#[derive(Debug)]
pub struct AspName(pub String);

impl AspName {
    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
        logger.enter_parser("AspName")?;
        logger.leave_parser("AspName")
    }
}
