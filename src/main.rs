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

mod shell {
    use std::fmt::Debug;

    pub struct Input {
        command: Command,
        args: Vec<String>,
    }

    impl From<String> for Input {
        fn from(input: String) -> Self {
            let mut args = input.split_whitespace();
            let command = match args.next().map(|s| s.to_lowercase()) {
                Some(s) if s == "exit" || s == "quit" => Command::Exit,
                _ => Command::None,
            };

            Self {
                command,
                args: args.map(|s| s.to_string()).collect(),
            }
        }
    }
    #[derive(Debug)]
    enum Command {
        None,
        Exit,
    }

    #[derive(PartialEq)]
    pub enum ReturnCode {
        Success,
        Exit,
    }

    pub fn print_welcome_message() {
        println!(
            "Welcome to rush! Current time is {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );
    }

    pub fn process_input(input: Input) -> ReturnCode {
        match input.command {
            Command::Exit => ReturnCode::Exit,
            _ => ReturnCode::Success,
        }
    }
}
