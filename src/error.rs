use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Supported formats: 'gclog', 'logfmt', 'clf'.")]
    InvalidLogFmtType(String),
    #[error("Supported output formats: 'csv', 'tsv', 'ndjson'.")]
    InvalidOutFmtType(String),
}
