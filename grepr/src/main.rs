fn main() -> grepr::MyResult<()> {
    grepr::run(&grepr::get_args())?;
    Ok(())
}
