use std::error::Error;

use clap::{Parser, command, crate_authors, crate_version, ValueEnum};
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

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

    for path in &config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => if matches_types(&entry, &config.entry_types) &&
                    matches_patterns(&entry, &config.names) {
                    println!("{}", entry.path().display())
                },
            }
        }
    }
    Ok(())
}

fn matches_patterns(entry: &DirEntry, regexs: &[Regex]) -> bool {
    let name = entry.file_name().to_str();
    match name {
        None => false,
        Some(name) => regexs.is_empty() || regexs.iter().any(|regex| { regex.is_match(name) }),
    }
}


fn matches_types(entry: &DirEntry, entry_types: &[EntryType]) -> bool {
    if entry_types.is_empty() {
        true
    } else {
        match get_entry_type(entry) {
            Some(entry_type) => entry_types.contains(&entry_type),
            None => false
        }
    }
}

fn get_entry_type(entry: &DirEntry) -> Option<EntryType> {
    let file_type = entry.file_type();

    if file_type.is_file() {
        Some(EntryType::File)
    } else if file_type.is_dir() {
        Some(EntryType::Dir)
    } else if file_type.is_symlink() {
        Some(EntryType::Link)
    } else {
        None
    }
}


fn validate_regex(s: &str) -> Result<Regex, String> {
    Regex::new(s).map_err(|_|{ format!("Invalid --name \"{}\"", s)})
}
