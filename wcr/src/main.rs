use clap::Parser;
use wcr::{run, Config, MyResult};

fn main() -> MyResult<()> {
    
    run(&Config::parse())

}
