use std::{error::Error, fmt::Debug, path::Path};

use clap::{Parser, command, crate_authors, crate_version, ArgAction};
use regex::{RegexBuilder, Regex};
use walkdir::{WalkDir, DirEntry};

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

    for result in find_files(&config.files, config.recursive) {
        println!("{:?}", result);
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

    let path = Path::new(file_path);
    let mut results = vec![];

    if !path.exists() {
        let error_message = format!("file {} does not exist", file_path);
        return vec![Err(Box::new(MyError{ error_message }))];
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

// --------------------------------------------------
#[cfg(test)]
mod tests {

    use super::find_files;
    use rand::{distributions::Alphanumeric, Rng};

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

}
