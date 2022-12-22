use std::{error::Error, io::{BufRead, BufReader, self, Write, stdout}, fs::File, path::Path};
use clap::{command, Parser, crate_authors, crate_version, ArgAction};

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = "Rust version of uniq"
)]
pub struct Config {

    #[arg(
        action = ArgAction::Set,
        required = false,
        default_value = "-",
        help = "Input file"
    )]
    in_file: String,

    #[arg(
        action = ArgAction::Set,
        required = false,
        default_value = None,
        help = "Output file"
    )]
    out_file: Option<String>,

    #[arg(
        short = 'c',
        long = "count",
        action = ArgAction::SetTrue,
        help = "Show counts"
    )]
    count: bool,

}

pub type MyResult<T> = Result<T, Box<dyn Error>>;


pub fn get_args() -> Config {
    Config::parse()
}

pub fn run(config: &Config) -> MyResult<()> {

    let input = match open(&config.in_file) {
        Ok(input) => input,
        Err(err) => return Err(Box::<dyn Error>::from(
            format!("{}: {}", &config.in_file, err))),
    };

    process_input(input, &config.out_file, config.count)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn process_input(
    mut in_file: impl BufRead,
    out_filename: &Option<String>,
    show_count: bool) -> MyResult<()> {

    let mut line = String::new();
    let mut last_line: Option<String> = None;
    let mut counter = 0usize;
    let mut output: Box<dyn Write> = if out_filename.is_some() {
        let filename = out_filename.clone().unwrap();
        let path = Path::new(&filename);
        Box::new(File::create(path)?)
    } else {
        Box::new(stdout())
    };

    while let Ok(bytes_cnt) = &in_file.read_line(&mut line) {
        if bytes_cnt == &0 {
            break;
        }
        if last_line.is_none() {
            last_line = Some(line.clone());
            counter = 1;
        } else if strings_differ(&last_line.clone().unwrap(), &line) {
            if !show_count {
                write!(output, "{}", last_line.unwrap())?;
            } else {
                write!(output, "{:>4} {}", counter, last_line.unwrap())?;
            }
            last_line = Some(line.clone());
            counter = 1;
        } else {
            counter += 1;
        }

        line.clear();
    }

    if last_line.is_some() {
        if !show_count {
            write!(output, "{}", last_line.unwrap())?;
        } else {
            write!(output, "{:>4} {}", counter, last_line.unwrap())?;
        }
    }

    Ok(())
}

fn strings_differ(s1: &str, s2: &str) -> bool {
    s1.trim_end() != s2.trim_end()
}
