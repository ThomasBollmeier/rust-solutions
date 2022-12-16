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
        .args(["chars","bytes"])
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

    let mut num_files = 0;
    let mut totals = FileInfo{
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };

    for filename in &config.files {
        match open(filename) {
            Ok(file) => {
                let file_info = count(file)?;
                num_files += 1;
                totals.num_lines += file_info.num_lines;
                totals.num_words += file_info.num_words;
                totals.num_bytes += file_info.num_bytes;
                totals.num_chars += file_info.num_chars;
                let message = compose_message(&file_info, config);
                if filename != "-" {
                    println!("{} {}", message, filename);
                } else {
                    println!("{}", message);
                }
            },
            Err(error) => eprintln!("{}: {}", filename, error), 
        }
    }

    if num_files > 1 {
        let message = compose_message(&totals, config);
        println!("{} total", message);
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn compose_message(file_info: &FileInfo, config: &Config) -> String {
    let mut ret = " ".to_string();
    let mut is_first = true;

    if config.lines {
        is_first = false;
        ret.push_str(&format!("{:>7}", file_info.num_lines));
    }
    if config.words {
        if !is_first {
            ret.push_str(" ");
        } else {
            is_first = false;
        }
        ret.push_str(&format!("{:>7}", file_info.num_words));
    }
    if config.bytes {
        if !is_first {
            ret.push_str(" ");
        } else {
            is_first = false;
        }
        ret.push_str(&format!("{:>7}", file_info.num_bytes));
    }
    if config.chars {
        if !is_first {
            ret.push_str(" ");
        }
        ret.push_str(&format!("{:>7}", file_info.num_chars));
    }

    ret
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0usize;
    let mut num_words = 0usize;
    let mut num_bytes = 0usize;
    let mut num_chars = 0usize;

    let mut line = String::new();

    while let Ok(bytes_cnt) = &file.read_line(&mut line) {
        if bytes_cnt == &0 {
            break;
        }
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += bytes_cnt;
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo { 
        num_lines, 
        num_words, 
        num_bytes, 
        num_chars,
    })

}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{FileInfo, count};

    #[test]
    fn test_count() {   
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }

}