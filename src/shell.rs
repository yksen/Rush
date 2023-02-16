mod commands;

use std::io::{self, Write};

#[derive(Debug)]
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

pub fn init() -> io::Result<()> {
    print_welcome_message();
    shell_loop()?;
    Ok(())
}

fn print_welcome_message() {
    println!(
        "Welcome to rush! Current time is {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
}

fn shell_loop() -> Result<(), io::Error> {
    let mut input = String::new();
    while process_input(input.clone().into()) != ReturnCode::Exit {
        input.clear();
        print_command_prompt()?;
        io::stdin().read_line(&mut input)?;
    }
    Ok(())
}

fn print_command_prompt() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    Ok(())
}

fn process_input(input: Input) -> ReturnCode {
    match input.command.as_str() {
        "exit" | "quit" => commands::exit(),
        _ => ReturnCode::Success,
    }
}
