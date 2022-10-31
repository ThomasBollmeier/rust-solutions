use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{Parser, ArgAction, crate_authors, crate_version};


#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"), 
    version = crate_version!(), 
    about = "Rust version of cat"
)]
pub struct Config {

    #[arg(
        action = ArgAction::Append, 
        required = false,
        default_value = "-",
        help = "Input file(s)"
    )]
    files: Vec<String>,
    
    #[arg(
        short, 
        long = "number",
        action = ArgAction::SetTrue,
        conflicts_with = "number_nonblank_lines",
        help = "Number lines"
    )]
    number_lines: bool,
    
    #[arg(
        short = 'b', 
        long = "number-nonblank",
        action = ArgAction::SetTrue,
        help = "Number nonblank lines"
    )]
    number_nonblank_lines: bool,
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: &Config) -> MyResult<()> {
    
    for filename in &config.files {
        match open(filename) {
            Ok(file) => print_file_content(file, config),
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

fn print_file_content(file: Box<dyn BufRead>, config: &Config) {

    if !config.number_lines && !config.number_nonblank_lines {

        for line in file.lines() {
            if let Ok(line) = line {
                println!("{}", line);            
            }
        }
    
    } else if config.number_lines {

        for (line_num, line) in file.lines().enumerate() {
            if let Ok(line) = line {
                println!("{:>6}\t{}", line_num + 1, line);            
            }
        }

    } else if config.number_nonblank_lines {

        let mut line_num = 1;

        for line in file.lines() {
            if let Ok(line) = line {
                if !line.is_empty() {
                    println!("{:>6}\t{}", line_num, line);
                    line_num += 1;       
                } else {
                    println!("{}", line);
                }    
            }
        }

    }

}