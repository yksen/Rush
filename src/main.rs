use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    while shell::process_input(input.clone()) != shell::ExitCode::Exit {
        input.clear();
        io::stdin().read_line(&mut input)?;
    }

    Ok(())
}

mod shell {
    #[derive(PartialEq)]
    pub enum ExitCode {
        Success,
        Exit,
    }

    pub enum Command {
        None,
        Exit,
    }

    pub fn process_input(input: String) -> ExitCode {
        let command = parse_input(input);

        match command {
            Command::Exit => ExitCode::Exit,
            _ => ExitCode::Success,
        }
    }

    fn parse_input(input: String) -> Command {
        let command = input.trim().to_lowercase();

        match command.as_str() {
            "exit" => Command::Exit,
            _ => Command::None,
        }
    }
}
