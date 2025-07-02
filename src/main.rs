use clap::Parser;
use riplog::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
