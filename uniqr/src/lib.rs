use std::error::Error;

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
        required = true,
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

    println!("{:?}", config);
    Ok(())
}
