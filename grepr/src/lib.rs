use std::error::Error;

use clap::{Parser, command, crate_authors, crate_version, ArgAction};
use regex::RegexBuilder;
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
        value_parser = validate_regex,
    )]
    pattern: String,

    #[arg(
        value_name = "FILE",
        help = "Input file(s)",
        default_value = "-",
        num_args = 1..,
    )]
    files: Vec<String>,

    #[arg(
        short,
        long = "insensitive",
        action = ArgAction::SetTrue,
        help = "Case-insensitive"
    )]
    insensitive: bool,

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
        short = 'v',
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

fn validate_regex(pattern: &str) -> Result<String, String> {
    match RegexBuilder::new(pattern).build() {
        Ok(_regex) => Ok(pattern.to_string()),
        Err(_err) => Err(format!("Invalid pattern \"{}\"", pattern)),
    }
}
