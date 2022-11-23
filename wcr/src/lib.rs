use std::error::Error;

use clap::{command, Parser, crate_authors, crate_version, ArgAction};

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"), 
    version = crate_version!(), 
    about = "Rust version of wc"
)]
pub struct Config {

    #[arg(
        action = ArgAction::Append, 
        required = false,
        default_value = "-",
        help = "Input file(s)"
    )]
    files: Vec<String>,
    
    #[arg(
        short = 'l', 
        long = "lines",
        action = ArgAction::SetTrue,
        help = "Show line count"
    )]
    lines: bool,

    #[arg(
        short = 'w', 
        long = "words",
        action = ArgAction::SetTrue,
        help = "Show word count"
    )]
    words: bool,

    #[arg(
        short = 'c', 
        long = "bytes",
        action = ArgAction::SetTrue,
        help = "Show byte count"
    )]
    bytes: bool,

    #[arg(
        short = 'm', 
        long = "chars",
        action = ArgAction::SetTrue,
        help = "Show character count"
    )]
    chars: bool,
    
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: &Config) -> MyResult<()> {

    println!("{:#?}", config);

    Ok(())
}