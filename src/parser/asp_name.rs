use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct AspName(pub String);

impl AspName {
    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        for _ in 0..=(indentation * 2) { file.write(b" ")?; }
        file.write(b"<AspName>\n")?;

        for _ in 0..=(indentation * 2) { file.write(b" ")?; }
        file.write(b"<AspName/>\n")?;
        Ok(())
    }
}
