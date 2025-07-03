use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Unsupported log format '{0}'. Supported formats: 'gclog', 'logfmt', 'clf'.")]
    InvalidLogFmtType(String),
    #[error("Unsupported output format '{0}'. Supported output formats: 'csv', 'tsv', 'ndjson'.")]
    InvalidOutFmtType(String),
}
