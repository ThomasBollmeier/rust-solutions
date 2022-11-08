use std::error::Error;
use clap::{Parser, ArgAction, ArgGroup, crate_authors, crate_version};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: &Config) -> MyResult<()> {

    println!("{:#?}", config);

    Ok(())
}

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"), 
    version = crate_version!(), 
    about = "Rust version of head"
)]
#[command(group(
    ArgGroup::new("header_size")
        .args(["lines", "bytes"])
))]
pub struct Config {

    #[arg(
        action = ArgAction::Append, 
        required = false,
        default_value = "-",
        help = "Input file(s)"
    )]
    files: Vec<String>,
    
    #[arg(
        short = 'n', 
        long = "lines",
        value_parser = validate_lines,
        action = ArgAction::Set,
        default_value = "10",
        help = "Number of lines to print"
    )]
    lines: usize,
    
    #[arg(
        short = 'c', 
        long = "bytes",
        value_parser = validate_bytes,
        action = ArgAction::Set,
        help = "Number of bytes to print"
    )]
    bytes: Option<usize>,
}

fn validate_lines(s: &str) -> Result<usize, String> {

    match s.parse::<usize>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("illegal line count -- {}", s))
    }

}

fn validate_bytes(s: &str) -> Result<usize, String> {

    match s.parse::<usize>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("illegal byte count -- {}", s))
    }

}
