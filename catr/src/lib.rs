use std::error::Error;
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
        action = ArgAction::SetTrue,
        help = "Number lines"
    )]
    number_lines: bool,
    
    #[arg(
        short = 'b', 
        action = ArgAction::SetTrue,
        help = "Number nonblank lines"
    )]
    number_nonblank_lines: bool,
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: &Config) -> MyResult<()> {
    
    println!("{:#?}", config);
    
    Ok(())
}