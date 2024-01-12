
use clap::{command, Parser, builder::OsStr};
use regex::Regex;

use crate::Offset::*;
use std::error::Error;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Clone)]
enum Offset {
    FromStart(u64),
    FromEnd(u64),
}

impl From<Offset> for OsStr {
    fn from(value: Offset) -> Self {
        match value {
            FromStart(num) => OsStr::from(format!("+{}", num.to_string())),
            FromEnd(num) => OsStr::from(num.to_string())
        }
    }
}

fn parse_lines(s: &str) -> Result<Offset, String> {
    match parse_offset(s) {
        Some(offset) => Ok(offset),
        None => Err(format!("illegal line count -- {}", s))
    }
}

fn parse_bytes(s: &str) -> Result<Offset, String> {
    match parse_offset(s) {
        Some(offset) => Ok(offset),
        None => Err(format!("illegal byte count -- {}", s))
    }
}

fn parse_offset(s: &str) -> Option<Offset> {

    let regex = Regex::new(r"^(\+)?(\d+)$").unwrap();
    match regex.captures(s) {
        Some(captures) => {
            let from_start = captures.get(1).is_some();
            let num_str = captures.get(2).unwrap().as_str();
            let num = num_str.parse().unwrap();
            if from_start {
                Some(FromStart(num))
            } else {
                Some(FromEnd(num))
            }
        },
        None => None,
    }
}

#[derive(Debug, Parser)]
#[command(
    author = "Thomas Bollmeier",
    version = "0.1.0",
    about = "Rust version of tail"
)]
pub struct Config {
    #[arg(
        value_name = "FILE",
        help = "Input file(s)",
        num_args = 1..,
        required = true
    )]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "lines",
        group = "mode",
        value_name = "LINES",
        help = "Number of lines",
        value_parser = parse_lines,
        default_value = FromEnd(10)
    )]
    lines: Offset,

    #[arg(
        short = 'c',
        long = "bytes",
        group = "mode",
        value_name = "BYTES",
        help = "Number of bytes",
        value_parser = parse_bytes
    )]
    bytes: Option<Offset>,

    #[arg(
        short = 'q',
        long = "quiet",
        help = "Suppress headers"
    )]
    quiet: bool,
}

pub fn get_config() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {

    println!("{:?}", config);

    Ok(())
}
