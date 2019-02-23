use crate::log::logger::Logger;
use std::io;

#[derive(Debug)]
pub struct AspString(pub String);

impl AspString {
    pub fn test_parser(&self, logger: &mut Logger) -> io::Result<()> {
        logger.enter_parser("AspString")?;
        logger.leave_parser("AspString")
    }
}
