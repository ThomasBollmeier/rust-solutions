use commr::{get_config, run, MyResult};

fn main() -> MyResult<()> {
    let config = get_config()?;
    run(&config)?;
    Ok(())
}
