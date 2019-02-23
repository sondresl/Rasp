use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct AspName(pub String);

impl AspName {
    pub fn test_parser(&self, file: &mut File, indentation: u32) -> std::io::Result<()> {
        file.write_all(b"<AspName>");
        Ok(())
    }
}
