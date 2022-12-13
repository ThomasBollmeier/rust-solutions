use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};

use clap::{command, Parser, crate_authors, crate_version, ArgAction, ArgGroup};

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"), 
    version = crate_version!(), 
    about = "Rust version of wc"
)]
#[command(group(
    ArgGroup::new("mode")
        .args(["bytes", "chars"])
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

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> Config {
    let mut config = Config::parse();

    if [config.lines, config.words, config.bytes, config.chars].iter().all(|v|{ v == &false}) {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }

    config
}

pub fn run(config: &Config) -> MyResult<()> {

    for filename in &config.files {
        match open(filename) {
            Ok(file) => {
                count(file)?;
                ()
            },
            Err(error) => eprintln!("{}: {}", filename, error), 
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut _file: impl BufRead) -> MyResult<FileInfo> {
    let num_lines = 0usize;
    let num_words = 0usize;
    let num_bytes = 0usize;
    let num_chars = 0usize;

    Ok(FileInfo { 
        num_lines, 
        num_words, 
        num_bytes, 
        num_chars,
    })

}