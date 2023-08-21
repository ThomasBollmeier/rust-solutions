fn main() -> grepr::MyResult<()> {
    let config = grepr::get_config()?;
    grepr::run(&config)?;
    Ok(())
}
