use crate::log::logger::Logger;

#[derive(Debug)]
pub struct AspName(pub String);

impl AspName {
    pub fn test_parser(&self, logger: &mut Logger) -> std::io::Result<()> {
        logger.enter_parser("AspName")?;
        logger.leave_parse("AspName")
    }
}
