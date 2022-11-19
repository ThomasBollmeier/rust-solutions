use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};
use clap::{Parser, ArgAction, ArgGroup, crate_authors, crate_version};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: &Config) -> MyResult<()> {

    let is_title_printed = config.files.len() > 1;
    let mut is_first = true;

    for filename in &config.files {

        if !is_first {
            println!();
        } else {
            is_first = false;
        }

        match open(filename) {
            Ok(file) => print_header_lines(file, is_title_printed, filename, config),
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
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

fn print_header_lines(file: Box<dyn BufRead>, is_title_printed: bool, filename: &str,
    config: &Config) {

    if is_title_printed {
        println!("==> {} <==", filename);
    }

    match config.bytes {
        Some(num_bytes) => print_n_first_bytes(file, num_bytes),
        None => print_n_first_lines(file, config.lines),
    }
}

fn print_n_first_lines(mut file: Box<dyn BufRead>, num_lines: usize) {
    
    let mut line = String::new();
    let mut num_lines_read = 0;

    while num_lines_read < num_lines {

        match file.read_line(&mut line) {
            Ok(num_bytes) => if num_bytes > 0 {
                print!("{}", line);
                num_lines_read += 1;
                line.clear();
            } else {
                break;
            },
            Err(_) => break,
        }

    }

}

fn print_n_first_bytes(mut file: Box<dyn BufRead>, num_bytes: usize) {

    let mut bytes_read: Vec<u8> = Vec::new();
    let mut buffer = [0u8; 1];

    while bytes_read.len() < num_bytes {
        if let Ok(n) = file.read(&mut buffer) {
            if n > 0 {
                bytes_read.push(buffer[0]);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let s = String::from_utf8_lossy(&bytes_read[..]);
    print!("{}", s);
}

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"), 
    version = crate_version!(), 
    about = "Rust version of head"
)]
#[command(group(
    ArgGroup::new("header_size")
        .args(["lines", "bytes"])
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
        short = 'n', 
        long = "lines",
        value_parser = validate_lines,
        action = ArgAction::Set,
        default_value = "10",
        help = "Number of lines to print"
    )]
    lines: usize,
    
    #[arg(
        short = 'c', 
        long = "bytes",
        value_parser = validate_bytes,
        action = ArgAction::Set,
        help = "Number of bytes to print"
    )]
    bytes: Option<usize>,
}

fn validate_lines(s: &str) -> Result<usize, String> {
    validate(s, format!("illegal line count -- {}", s))
}

fn validate_bytes(s: &str) -> Result<usize, String> {
    validate(s, format!("illegal byte count -- {}", s))
}

fn validate(s: &str, message: String) -> Result<usize, String> {

    match s.parse::<usize>() {
        Ok(value) => Ok(value),
        Err(_) => Err(message)
    }

}