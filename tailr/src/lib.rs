
use clap::{command, Parser, builder::OsStr};
use once_cell::sync::OnceCell;
use regex::Regex;

use crate::Offset::*;
use std::{error::Error, fs::File, fmt::Debug,
    io::{BufRead, BufReader, Read, Seek}};

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


#[derive(Debug, PartialEq, Clone)]
enum Offset {
    Start(u64),
    End(u64),
}

impl From<Offset> for OsStr {
    fn from(value: Offset) -> Self {
        match value {
            Start(num) => OsStr::from(format!("+{}", num.to_string())),
            End(num) => OsStr::from(num.to_string())
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

static OFFSET_REGEX: OnceCell<Regex> = OnceCell::new();

fn parse_offset(s: &str) -> Option<Offset> {

    let regex = OFFSET_REGEX.get_or_init(
        || { Regex::new(r"^([+-])?(\d+)$").unwrap() });

    match regex.captures(s) {
        Some(captures) => {
            let from_start = match captures.get(1) {
                Some(m) => m.as_str() == "+",
                None => false,
            };
            let num_str = captures.get(2).unwrap().as_str();
            let num = num_str.parse().unwrap();
            if from_start {
                Some(Start(num))
            } else {
                Some(End(num))
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
        default_value = End(10)
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

    let multiple_files = config.files.len() > 1;

    for (file_num, filename) in config.files.iter().enumerate() {
        match File::open(&filename) {
            Ok(file) => {
                if multiple_files && !config.quiet {
                    println!("{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename);
                }
                let (num_lines, num_bytes) =
                    count_lines_bytes(&filename)?;
                run_file(file, &config, num_lines, num_bytes);
            }
            Err(error) => eprintln!("{}: {}", filename, error)
        }
    }

    Ok(())
}

fn run_file(
    file: File,
    config: &Config,
    num_lines: u64,
    num_bytes: u64
) {
    if let Some(bytes_offset) = &config.bytes {
        print_bytes(BufReader::new(file), bytes_offset, num_bytes);
    } else {
        print_lines(BufReader::new(file), &config.lines, num_lines);
    }
}

fn count_lines_bytes(filename: &str) -> MyResult<(u64, u64)> {

    let file = File::open(&filename).map_err(|e| {
            Box::new(MyError { error_message: format!("{}", e)})
        })?;

    let metadata = file.metadata()?;
    let num_bytes = metadata.len();

    let num_lines = BufReader::new(file).lines().count() as u64;

    Ok((num_lines, num_bytes))
}

fn print_lines(
    mut file: impl BufRead,
    offset: &Offset,
    num_lines: u64
) {
    if let Some(start_idx) = get_start_index(offset, num_lines) {

        let mut line = String::new();
        let mut idx = 0;

        while let Ok(size) = file.read_line(&mut line) {
            if size == 0 {
                break;
            }
            if idx >= start_idx {
                print!("{}", line);
            }
            idx += 1;
            line.clear();
        }
    }
}

fn print_bytes<T: Read + Seek>(
    mut file : T,
    offset: &Offset,
    num_bytes: u64
) {
    if let Some(start_idx) = get_start_index(offset, num_bytes)  {
        file.seek(std::io::SeekFrom::Start(start_idx)).expect("seek failed");
        let mut bytes: Vec<u8> = vec![];
        if let Ok(_) = file.read_to_end(&mut bytes) {
            let content = String::from_utf8_lossy(&bytes);
            print!("{}", content);
        }
    }
}

fn get_start_index(offset: &Offset, size: u64) -> Option<u64> {
    if size == 0 {
        return None;
    }

    match offset {
        Start(0) => Some(0),
        Start(num) => if *num <= size {
            Some(num - 1)
        } else {
            None
        },
        End(0) => None,
        End(num) => {
            let start_idx = if size >= *num {
                size - num
            } else {
                0
            };
            Some(start_idx)
        },
    }
}


// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        parse_offset, Offset::*, count_lines_bytes, get_start_index,
    };


    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 24));

        let res = count_lines_bytes("tests/inputs/ten.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (10, 49));
    }

    #[test]
    fn test_get_start_index() {
        // +0 from an empty file (0 lines/bytes) returns None
        assert_eq!(get_start_index(&Start(0), 0), None);

        // +0 from a nonempty file returns an index that
        // is one less than the number of lines/bytes
        assert_eq!(get_start_index(&Start(0), 1), Some(0));

        // Taking 0 lines/bytes returns None
        assert_eq!(get_start_index(&End(0), 1), None);

        // Taking any lines/bytes from an empty file returns None
        assert_eq!(get_start_index(&Start(1), 0), None);

        // Taking more lines/bytes than is available returns None
        assert_eq!(get_start_index(&Start(2), 1), None);

        // When starting line/byte is less than total lines/bytes,
        // return one less than starting number
        assert_eq!(get_start_index(&Start(1), 10), Some(0));
        assert_eq!(get_start_index(&Start(2), 10), Some(1));
        assert_eq!(get_start_index(&Start(3), 10), Some(2));

        // When starting line/byte is negative and less than total,
        // return total - start
        assert_eq!(get_start_index(&End(1), 10), Some(9));
        assert_eq!(get_start_index(&End(2), 10), Some(8));
        assert_eq!(get_start_index(&End(3), 10), Some(7));

        // When the starting line/byte is negative and more than the total,
        // return 0 to print the whole file
        assert_eq!(get_start_index(&End(20), 10), Some(0));
    }

    #[test]
    fn test_parse_offset() {
        // All integers should be interpreted as negative numbers
        let res = parse_offset("3");
        assert!(res.is_some());
        assert_eq!(res.unwrap(), End(3));

        // A leading "+" should result in a positive number
        let res = parse_offset("+3");
        assert!(res.is_some());
        assert_eq!(res.unwrap(), Start(3));

        // An explicit "-" value should result in a negative number
        let res = parse_offset("-3");
        assert!(res.is_some());
        assert_eq!(res.unwrap(), End(3));

        // Zero is zero
        let res = parse_offset("0");
        assert!(res.is_some());
        assert_eq!(res.unwrap(), End(0));

        // Plus zero is special
        let res = parse_offset("+0");
        assert!(res.is_some());
        assert_eq!(res.unwrap(), Start(0));

        // Test boundaries
        let res = parse_offset(&u64::MAX.to_string());
        assert!(res.is_some());
        assert_eq!(res.unwrap(), End(u64::MAX));

        let res = parse_offset(&format!("+{}", u64::MAX));
        assert!(res.is_some());
        assert_eq!(res.unwrap(), Start(u64::MAX));

        // A floating-point value is invalid
        let res = parse_offset("3.14");
        assert!(res.is_none());

        // Any non-integer string is invalid
        let res = parse_offset("foo");
        assert!(res.is_none());
    }
}
