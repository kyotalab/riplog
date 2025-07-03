use anyhow::Result;
use clap::Parser;
use riplog::{Args, GclogParser, LogFmtParser};

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let parser = GclogParser;

    let reader = parser.read(args.input)?;
    parser.parse(reader)?;
    
    Ok(())
}
