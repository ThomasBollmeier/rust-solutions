use cutr::{get_config, run};

fn main() {

  if let Err(error) = get_config().and_then(run) {
    eprintln!("{}", error);
    std::process::exit(1);
  }

}
