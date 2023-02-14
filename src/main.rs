mod shell;

use std::io;

fn main() -> io::Result<()> {
    shell::print_welcome_message();

    let mut input = String::new();
    while shell::process_input(input.clone().into()) != shell::ReturnCode::Exit {
        input.clear();
        io::stdin().read_line(&mut input)?;
    }

    Ok(())
}
