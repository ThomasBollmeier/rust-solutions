use std::error::Error;

use clap::{Parser, command, crate_authors, crate_version, ArgAction};
use regex::Regex;
//use walkdir::{WalkDir, DirEntry};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = "Rust version of grep"
)]
pub struct Config {
    #[arg(
        value_name = "PATTERN",
        help = "Search pattern",
        num_args = 1,
    )]
    pattern: Regex,

    #[arg(
        value_name = "FILE",
        help = "Input file(s)",
        default_value = "-",
        num_args = 1..,
    )]
    files: Vec<String>,

    #[arg(
        short,
        long = "recursive",
        action = ArgAction::SetTrue,
        help = "Recursive search"
    )]
    recursive: bool,

    #[arg(
        short,
        long = "count",
        action = ArgAction::SetTrue,
        help = "Count occurrences"
    )]
    count: bool,

    #[arg(
        short,
        long = "invert-match",
        action = ArgAction::SetTrue,
        help = "Invert match"
    )]
    invert_match: bool,
}

pub fn get_args() -> Config {
    Config::parse()
}

pub fn run(config: &Config) -> MyResult<()> {

    println!("{:#?}", config);

    Ok(())
}
