use std::{error::Error, fmt::Debug, io::{BufRead, BufReader, self}, fs::File, vec, cmp::Ordering};

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

    let lines1 = read_file_content(file1)?;
    let lines2 = read_file_content(file2)?;

    print_line_diffs(&lines1, &lines2, &config);

    Ok(())
}

fn print_line_diffs(lines1: &[String], lines2: &[String], config: &Config) {

    let n1 = lines1.len();
    let mut i1 = 0;
    let n2 = lines2.len();
    let mut i2 = 0;

    while i1 < n1 || i2 < n2 {
        if i1 >= n1 {
            print_col2(&lines2[i2], config);
            i2 += 1;
            continue;
        } else if i2 >= n2 {
            print_col1(&lines1[i1], config);
            i1 += 1;
            continue;
        }

        let mut line1 = lines1[i1].to_string();
        let mut line2 = lines2[i2].to_string();

        if config.insensitive {
            line1 = line1.to_lowercase();
            line2 = line2.to_lowercase();
        }

        let order = line1.cmp(&line2);

        match order {
            Ordering::Less => {
                print_col1(&line1, config);
                i1 += 1;
            }
            Ordering::Greater => {
                print_col2(&line2, config);
                i2 += 1;
            }
            Ordering::Equal => {
                print_col3(&line1, config);
                i1 += 1;
                i2 += 1;
            }
        }

    }

}

fn print_col1(line: &str, config: &Config) {
    if config.show_col1 {
        println!("{}", line);
    }
}

fn print_col2(line: &str, config: &Config) {
    if !config.show_col2 {
        return;
    }

    let mut tabs = "".to_string();
    if config.show_col1 {
        tabs.push_str(&config.delimiter);
    }

    println!("{}{}", tabs, line);
}

fn print_col3(line: &str, config: &Config) {
    if !config.show_col3 {
        return;
    }

    let mut tabs = "".to_string();
    if config.show_col1 {
        tabs.push_str(&config.delimiter);
    }
    if config.show_col2 {
        tabs.push_str(&config.delimiter);
    }

    println!("{}{}", tabs, line);
}

fn read_file_content(filename:&str) -> MyResult<Vec<String>> {
    let file = open(filename)?;
    let lines = file.lines();
    let mut result: Vec<String> = vec![];

    for line in lines {
        let line = line?;
        result.push(line);
    }

    Ok(result)
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
