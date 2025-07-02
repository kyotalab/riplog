use std::{path::PathBuf, str::FromStr};
use clap::Parser;

use crate::CliError;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    #[arg(short, long)]
	pub fmt: Option<LogFmtType>,
    #[arg(short, long)]
    pub outfmt: Option<OutFmtType>,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub list_fmts: Option<bool>,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub pretty: Option<bool>
}

#[derive(Debug, Clone)]
pub enum LogFmtType {
    LogFmt,
    GcLog,
    Clf,
}

impl FromStr for LogFmtType {
    type Err = CliError;
   fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "logfmt" => Ok(LogFmtType::LogFmt),
            "gclog" => Ok(LogFmtType::GcLog),
            "clf" => Ok(LogFmtType::Clf),
            _ => Err(CliError::InvalidLogFmtType(s.into()))
        }
    }
}

#[derive(Debug, Clone)]
pub enum OutFmtType {
    Csv,
    Tsv,
    Ndjson,
}

impl FromStr for OutFmtType {
    type Err = CliError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(OutFmtType::Csv),
            "tsv" => Ok(OutFmtType::Tsv),
            "mdjson" => Ok(OutFmtType::Ndjson),
            _ => Err(CliError::InvalidOutFmtType(s.into()))
        }
    }
}
