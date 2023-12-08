use std::{error::Error, fmt::Debug, io::{BufRead, BufReader, self}, fs::File};

use clap::{command, Parser, crate_version, ArgAction};

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


#[derive(Debug, Parser)]
#[command(
    author = "Thomas Bollmeier",
    version = crate_version!(),
    about = "Rust version of comm"
)]
#[allow(dead_code)]
pub struct Config {
    #[arg(
        value_name = "FILE1",
        help = "Input file 1",
        num_args = 1,
    )]
    file1: String,

    #[arg(
        value_name = "FILE2",
        help = "Input file 2",
        num_args = 1,
    )]
    file2: String,

    #[arg(
        short = '1',
        action = ArgAction::SetFalse,
        help = "Suppress printing of column 1"
    )]
    show_col1: bool,

    #[arg(
        short = '2',
        action = ArgAction::SetFalse,
        help = "Suppress printing of column 2"
    )]
    show_col2: bool,

    #[arg(
        short = '3',
        action = ArgAction::SetFalse,
        help = "Suppress printing of column 3"
    )]
    show_col3: bool,

    #[arg(
        short,
        action = ArgAction::SetTrue,
        help = "Case-insensitive comparison of lines"
    )]
    insensitive: bool,

    #[arg(
        value_name = "DELIM",
        short,
        long = "output-delimiter",
        help = "Output delimiter",
        default_value = "\t"
    )]
    delimiter: String,
}

pub fn get_config() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: &Config) -> MyResult<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        let error_message = "Both input files cannot be STDIN (\"-\")".to_string();
        return Err(Box::new(MyError {error_message}));
    }

    let _file1 = open(file1)?;
    let _file2 = open(file2)?;
    println!("Opened {} and {}", file1, file2);

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => {
            let file = File::open(filename).map_err(|e| {
                format!("{}: {}", filename, e)
            })?;
            Ok(Box::new(BufReader::new(file)))
        },
    }
}
