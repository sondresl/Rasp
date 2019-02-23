use std::fs::File;
use std::io::Write;
use std::io;

pub struct Logger {
    file: File,
    indent: usize
}

impl Logger {

    pub fn new(filename: &str) -> io::Result<Logger> {
        Ok(Logger {
            file: File::create(filename)?,
            indent: 0
        })
    }

    pub fn enter_parser(&mut self, name: &str) -> io::Result<()> {
        self.file.write_fmt(format_args!("{}<{}>\n", str::repeat(" ", self.indent * 2), name))?;
        self.indent += 1;
        Ok(())
    }

    pub fn leave_parser(&mut self, name: &str) -> io::Result<()> {
        self.indent -= 1;
        self.file.write_fmt(format_args!("{}</{}>\n", str::repeat(" ", self.indent * 2), name))?;
        Ok(())
    }
}