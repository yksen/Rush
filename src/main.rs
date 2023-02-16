mod shell;

fn main() -> std::io::Result<()> {
    shell::init()?;
    Ok(())
}
