#[allow(unused_imports)]
use std::io::{self, Write};

const VALID_COMMANDS: [&str; 3] = ["echo", "type", "exit"];

fn handle_command(command: &str) {
    let tokens = command.split_whitespace().collect::<Vec<&str>>();
    match command {
        cmd if cmd.starts_with("echo") => {
            let rest = &tokens[1..];
            println!("{}", rest.join(" "));
        }
        cmd if cmd.starts_with("type") => {
            if tokens.len() > 1 {
                if VALID_COMMANDS.contains(&tokens[1]) {
                    println!("{} is a shell builtin", tokens[1]);
                } else {
                    println!("{}: not found", tokens[1]);
                }
            }
        }
        _ => {
            println!("{}: command not found", command);
        }
    }
}

fn main() {
    // by default, treat everything as invalid
    let mut exit = false;
    while !exit {
            // Wait for user input
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => exit = true,
            command => {
                handle_command(command);
            }
        }
    }
}
