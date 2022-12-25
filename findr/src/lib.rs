use std::error::Error;

use clap::{Parser, command, crate_authors, crate_version};
use regex::Regex;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = "Rust version of find"
)]
pub struct Config {
    paths: Vec<String>,
    #[arg(
        short = 'n',
        long = "name",
    )]
    names: Vec<Regex>,
    #[arg(
        short = 't',
        long = "type",
        value_parser = validate_entry_type,
    )]
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> Config {
    Config::parse()
}

pub fn run(config: &Config) -> MyResult<()> {

    println!("{:?}", config);
    Ok(())
}

fn validate_entry_type(s: &str) -> Result<EntryType, String> {
    if s.len() != 1 {
        return Err(format!("Unknown type: {}", s));
    }

    match s.chars().next().unwrap() {
        'f' => Ok(EntryType::File),
        'd' => Ok(EntryType::Dir),
        'l' => Ok(EntryType::Link),
        _ => Err(format!("Unknown type: {}", s)),
    }

}
