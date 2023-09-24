use std::{error::Error, fmt::Debug, path::Path, io::{BufRead, BufReader, self}, fs::File};

use clap::{Parser, command, crate_authors, crate_version, ArgAction};
use regex::{RegexBuilder, Regex};
use walkdir::WalkDir;

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
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = "Rust version of grep"
)]
struct Arguments {
    #[arg(
        value_name = "PATTERN",
        help = "Search pattern",
        num_args = 1,
    )]
    pattern: String,

    #[arg(
        value_name = "FILE",
        help = "Input file(s)",
        default_value = "-",
        num_args = 1..,
    )]
    files: Vec<String>,

    #[arg(
        short,
        long = "insensitive",
        action = ArgAction::SetTrue,
        help = "Case-insensitive"
    )]
    insensitive: bool,

    #[arg(
        short,
        long = "recursive",
        action = ArgAction::SetTrue,
        help = "Recursive search"
    )]
    recursive: bool,

    #[arg(
        short,
        long = "count",
        action = ArgAction::SetTrue,
        help = "Count occurrences"
    )]
    count: bool,

    #[arg(
        short = 'v',
        long = "invert-match",
        action = ArgAction::SetTrue,
        help = "Invert match"
    )]
    invert_match: bool,
}

#[derive(Debug)]
pub struct Config {
    pub pattern: Regex,
    pub files: Vec<String>,
    pub recursive: bool,
    pub count: bool,
    pub invert_match: bool,
}

pub fn get_config() -> MyResult<Config> {
    let args = Arguments::parse();
    let regex = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive)
        .build()
        .map_err(|_| {
            let error = MyError {
                error_message: format!("Invalid pattern \"{}\"", args.pattern),
            };
            Box::new(error)
        })?;

    Ok(Config {
        pattern: regex,
        files: args.files,
        recursive: args.recursive,
        count: args.count,
        invert_match: args.invert_match,
    })

}

pub fn run(config: &Config) -> MyResult<()> {

    let files = find_files(&config.files, config.recursive);
    let many_files = files.len() > 1;

    for result in files {

        match result {
            Ok(file_path) => {
                match open(&file_path) {
                    Ok(mut file) => {
                        let lines =
                            find_lines(&mut file, &config.pattern, config.invert_match)?;
                        let file_path_opt = if many_files {
                            Some(file_path.as_str())
                        } else {
                            None
                        };
                        print_result(&lines, file_path_opt, &config);
                    },
                    Err(e) => eprintln!("{}: {}", file_path, e),
                };
            },
            Err(e) => eprintln!("{}", e),
        }

    }

    Ok(())
}

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    let mut ret = vec![];
    for path in paths {
        ret.append(&mut find_files_in_path(path, recursive));
    }
    ret
}

fn find_files_in_path(file_path: &str, recursive: bool) -> Vec<MyResult<String>> {

    // Special handling for stdin
    if file_path == "-" {
        return vec![Ok("-".to_string())];
    }

    let path = Path::new(file_path);
    let mut results = vec![];

    match path.canonicalize() {
        Ok(_) => {},
        Err(error) => {
            let error_message = format!("{}: {}", file_path, error);
            return vec![Err(Box::new(MyError{ error_message }))];
        }
    }

    if path.is_dir() {
        if recursive {
            for entry in WalkDir::new(file_path).min_depth(1) {
                match entry {
                    Ok(dir_entry) => {
                        let dir_entry_results =
                            &mut find_files_in_path(dir_entry.path().to_str().unwrap(), recursive);
                        results.append(dir_entry_results);
                    }
                    Err(e) => {
                        let error_message = e.to_string();
                        let error: Box<dyn Error> = Box::new(MyError{ error_message });
                        results.push(Err(error));
                    }
                }
            }
        } else {
            let error_message = format!("{} is a directory", file_path);
            return vec![Err(Box::new(MyError{ error_message }))];
        }
    }

    if path.is_file() {
        results.push(Ok(file_path.to_owned()));
    }

    results
}

fn open(file_path: &str) -> MyResult<Box<dyn BufRead>> {
    match file_path {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_path)?))),
    }
}

fn find_lines(
    file: &mut impl BufRead,
    pattern: &Regex,
    invert_match: bool) -> MyResult<Vec<String>>
{
    let mut ret = vec![];
    let mut buf = String::new();

    while let Ok(num_bytes) = file.read_line(&mut buf) {
        if num_bytes > 0 {
            let line = buf.to_owned();
            let matched = pattern.is_match(&line);
            if matched != invert_match {
                ret.push(line);
            }
            buf.clear();
        } else {
            break;
        }
    }

    Ok(ret)
}

fn print_result(lines: &Vec<String>, file_path: Option<&str>, config: &Config) {
    let many_files = file_path.is_some();

    if !config.count {
        for line in lines {
            if !many_files {
                print!("{}", line);
            } else {
                print!("{}:{}", file_path.unwrap(), line);
            }
        }
    } else {
        if !many_files {
            println!("{}", lines.len());
        } else {
            println!("{}:{}", file_path.unwrap(), lines.len());
        }
    }
}

// --------------------------------------------------
#[cfg(test)]
mod tests {

    use super::{find_files, find_lines};
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files =
            find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());

    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(&mut Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(&mut Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(&mut Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(&mut Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }

}
