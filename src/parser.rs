use std::{collections::HashMap, io::BufRead, path::PathBuf};

use anyhow::{Error, Result};

pub trait LogFmtParser {
    // Logを読み込む
    fn read(&self, path: Option<PathBuf>) -> Result<Box<dyn BufRead>>;


    // ログを解析する
    fn parse(&self, reader: Box<dyn BufRead>) -> Result<()>;
}

pub struct LogRecord {
    pub fields: HashMap<String, String>,
}
