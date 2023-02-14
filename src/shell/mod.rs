mod commands;
pub struct Input {
    command: String,
    args: Vec<String>,
}

impl From<String> for Input {
    fn from(input: String) -> Self {
        let mut args = input.split_whitespace();
        let command = args.next().map(|s| s.to_lowercase()).unwrap_or_default();

        Self {
            command,
            args: args.map(|s| s.to_string()).collect(),
        }
    }
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
    match input.command.as_str() {
        "exit" | "quit" => commands::exit(),
        _ => ReturnCode::Success,
    }
}
