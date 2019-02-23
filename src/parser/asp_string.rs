use crate::log::logger::Logger;

#[derive(Debug)]
pub struct AspString(pub String);

impl AspString {
    pub fn test_parser(&self, logger: &mut Logger) -> std::io::Result<()> {
        logger.enter_parser("AspString")?;
        logger.leave_parse("AspString")
    }
}
