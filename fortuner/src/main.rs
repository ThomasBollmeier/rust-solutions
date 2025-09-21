use fortuner::Config;

fn main() {
    if let Err(e) = fortuner::Cli::new()
        .and_then(|cli| Config::try_from(cli))
        .and_then(|config| config.run()) {

        eprintln!("{e}");
        std::process::exit(1);
    }
}
