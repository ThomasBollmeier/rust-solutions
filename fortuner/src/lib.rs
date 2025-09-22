use std::error::Error;
use std::fmt::Debug;
use std::io::BufRead;
use std::path::Path;
use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

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
    )]
    seed: Option<String>,

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
        match self.pattern {
            Some(ref regex) => self.find_fortunes(regex)?,
            None => println!("No pattern"),
        }

        Ok(())
    }

    fn find_fortunes(&self, regex: &Regex) -> MyResult<()> {
        for source in &self.sources {
            let path = Path::new(source).canonicalize()?;
            if path.is_dir() {
                self.find_fortune_in_dir(path.to_str().unwrap(), regex)?;
            } else if path.is_file() {
                self.find_fortune_in_file(path.to_str().unwrap(), regex)?;
            }
        }

        Ok(())
    }

    fn find_fortune_in_dir(&self, dir_path: &str, regex: &Regex) -> MyResult<()> {
        for entry in WalkDir::new(dir_path) {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            if path_str == dir_path {
                continue;
            }
            if path.is_dir() {
                self.find_fortune_in_dir(path_str, regex)?;
            } else if path.is_file() {
                if path_str.contains(".") {
                    continue;
                }
                self.find_fortune_in_file(path_str, regex)?;
            }
        }

        Ok(())
    }

    fn find_fortune_in_file(&self, file_path: &str, regex: &Regex) -> MyResult<()> {
        let reader = match std::fs::File::open(file_path) {
            Ok(f) => std::io::BufReader::new(f),
            Err(e) => {
                let error_message = format!("{}: {}", file_path, e);
                return Err(Box::new(MyError { error_message }))
            },
        };

        let mut record : Vec<String> = Vec::new();
        let mut match_found = false;
        let mut record_done = false;
        let mut file_name_printed = false;

        for line in reader.lines() {
            let line = line?;
            if regex.is_match(&line) {
                match_found = true;
            }
            if line == "%" {
                record_done = true;
            }
            record.push(line);
            if record_done {
                if match_found {
                    if !file_name_printed {
                        let path = Path::new(file_path);
                        if let Some(name) = path.file_name() {
                            eprintln!("({})", name.to_str().unwrap());
                            eprintln!("%");
                        }
                        file_name_printed = true;
                    }

                    for rec_line in &record {
                        println!("{}", rec_line);
                    }
                }
                record.clear();
                match_found = false;
                record_done = false;
            }

        }

        Ok(())
    }
}

fn find_non_existing(files: &Vec<String>) -> Option<String> {
    for file in files {
        // Check if file or directory exists
        if !Path::new(file).exists() {
            return Some(file.to_string());
        }
    }

    None
}


impl TryFrom<Cli> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        let sources = match find_non_existing(&value.sources) {
            Some(f) => {
                let error_message = format!("{f}: No such file or directory (os error 2)");
                return Err(Box::new(MyError { error_message }))
            },
            None => value.sources,
        };
        let pattern = match value.pattern {
            Some(p) => {
                let regex = if value.case_insensitive {
                    Regex::new(&format!("(?i){}", p))
                } else {
                    Regex::new(&p)
                };
                match regex {
                    Ok(r) => Some(r),
                    Err(_) => {
                        let error_message = format!("Invalid --pattern \"{p}\"");
                        return Err(Box::new(MyError { error_message }))
                    },
                }
            }
            None => None,
        };
        let seed = match value.seed {
            Some(s) => {
                match parse_u64(&s) {
                    Ok(r) => Some(r),
                    Err(_) => {
                        let error_message = format!("invalid value '{s}' for '--seed <SEED>'");
                        return Err(Box::new(MyError { error_message }))
                    },
                }
            },
            None => None
        };
        Ok(Config { sources, pattern, seed })
    }
}

fn parse_u64(s: &str) -> MyResult<u64> {
    match s.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(Box::new(MyError { error_message: format!("\"{s}\" is not a valid integer") })),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u64() {
        let res = parse_u64("a");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("\"a\" is not a valid integer"));

        let res = parse_u64("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);

        let res = parse_u64("42");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 42);
    }
}