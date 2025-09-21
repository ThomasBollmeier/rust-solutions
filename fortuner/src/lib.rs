use std::collections::hash_map::VacantEntry;
use std::error::Error;
use std::fmt::Debug;
use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
#[command(
    author = "Thomas Bollmeier",
    version = "0.1.0",
    about = "Rust version of fortune"
)]
pub struct Cli {
    #[arg(
        value_name = "FILE",
        help = "Input files or directories",
        num_args = 1..,
        required = true
    )]
    sources: Vec<String>,

    #[arg(
        short = 'm',
        long = "pattern",
        value_name = "PATTERN",
        help = "Pattern")]
    pattern: Option<String>,

    #[arg(
        short = 's',
        long = "seed",
        value_name = "SEED",
        help = "Random seed",
        value_parser = clap::value_parser!(u64)
    )]
    seed: Option<u64>,

    #[arg(
        short = 'i',
        long = "insensitive",
        help = "Case insensitive pattern matching"
    )]
    case_insensitive: bool,
}

impl Cli {
    pub fn new() -> MyResult<Self> {
        match Cli::try_parse() {
            Ok(cli) => Ok(cli),
            Err(e) => Err(Box::new(MyError { error_message: e.to_string() })),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

impl Config {
    pub fn run(&self) -> MyResult<()> {
        println!("Running fortune with config: {self:?}");
        Ok(())
    }
}

impl TryFrom<Cli> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        let sources = value.sources;
        let pattern = match value.pattern {
            Some(p) => {
                let regex = if value.case_insensitive {
                    Regex::new(&format!("(?i){}", p))
                } else {
                    Regex::new(&p)
                };
                match regex {
                    Ok(r) => Some(r),
                    Err(e) => return Err(Box::new(MyError { error_message: e.to_string() })),
                }
            }
            None => None,
        };
        let seed = value.seed;
        Ok(Config { sources, pattern, seed })
    }
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;
pub struct MyError {
    error_message: String,
}

impl Debug for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl Error for MyError { }