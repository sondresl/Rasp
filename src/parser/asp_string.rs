use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct AspString(pub String);

impl AspString {
    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspString>\n")?;

        for _ in 0..=(indentation * 2) { file.write(b" ")?; };
        file.write(b"<AspString/>\n")?;
        Ok(())
    }
}
