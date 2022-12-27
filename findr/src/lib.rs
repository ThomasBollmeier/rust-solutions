use std::error::Error;

use clap::{Parser, command, crate_authors, crate_version, ValueEnum};
use regex::Regex;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
enum EntryType {
    #[value(name = "d")]
    Dir,
    #[value(name = "f")]
    File,
    #[value(name = "l")]
    Link,
}

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = "Rust version of find"
)]
pub struct Config {
    #[arg(
        value_name = "PATH",
        help = "Search paths",
        default_value = ".",
        num_args = 1..,
    )]
    paths: Vec<String>,

    #[arg(
        short = 'n',
        long = "name",
        value_name = "NAME",
        num_args = 1..,
        value_parser = validate_regex,
    )]
    names: Vec<Regex>,

    #[arg(
        short = 't',
        long = "type",
        value_name = "TYPE",
        value_enum,
        num_args = 1..,
    )]
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> Config {
    Config::parse()
}

pub fn run(config: &Config) -> MyResult<()> {

    println!("{:#?}", config);
    Ok(())
}

fn validate_regex(s: &str) -> Result<Regex, String> {
    println!("Validating: {}", s);
    Regex::new(s).map_err(|_|{ format!("Invalid --name \"{}\"", s)})
}
