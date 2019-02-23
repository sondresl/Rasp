use std::fs::File;
use std::io::Write;
use std::io;

pub struct Logger {
    file: File,
    indent: u32
}

impl Logger {

    pub fn new(filename: &str) -> io::Result<Logger> {
        Ok(Logger {
            file: File::create(filename)?,
            indent: 0
        })
    }

    pub fn enter_parser(&mut self, name: &str) -> io::Result<()> {
        for _ in 0..(self.indent * 2) { self.file.write(b" ")?; }
        self.file.write_fmt(format_args!("<{}>\n",name))?;
        self.indent += 1;

        Ok(())
    }

    pub fn leave_parse(&mut self, name: &str) -> io::Result<()> {
        self.indent -= 1;
        for _ in 0..(self.indent * 2) { self.file.write(b" ")?; }
        self.file.write_fmt(format_args!("</{}>\n",name))?;

        Ok(())
    }
}