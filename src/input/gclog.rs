use std::{fs::File, io::{self, BufRead, BufReader, Write}, path::PathBuf};

use anyhow::{Error, Result};

use crate::{LogFmtParser, LogRecord};


pub struct GclogParser;


impl LogFmtParser for GclogParser {
    fn read(&self, path: Option<PathBuf>) -> Result<Box<dyn BufRead>> {
        let reader: Box<dyn BufRead> = if let Some(ref log_file) = path {
            Box::new(BufReader::new(File::open(log_file)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };
        Ok(reader)
    }

    fn parse(&self, reader: Box<dyn BufRead>) -> Result<()> {
        let lines = reader.lines();
        let mut writer: Box<dyn Write> = Box::new(io::stdout());
        for line in lines.into_iter() {
            let record = line.unwrap_or("".to_string());
            writeln!(writer, "{}", record)?;
        }
        Ok(())
    }
}
