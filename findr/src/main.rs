fn main() -> findr::MyResult<()> {
    findr::run(&findr::get_args())?;
    Ok(())
}
