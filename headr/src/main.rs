use clap::Parser;
use headr::{MyResult, Config, run};

fn main() -> MyResult<()> {
    
    run(&Config::parse())
}

