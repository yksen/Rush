use std::io;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    println!("{}", user_input);

    Ok(())
}
