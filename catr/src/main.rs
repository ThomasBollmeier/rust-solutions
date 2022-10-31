use clap::Parser;

fn main() -> catr::MyResult<()> {

    let config = catr::Config::parse();

    catr::run(&config)
}
