#[allow(unused_imports)]
use std::io::{self, Write};

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
                if command.starts_with("echo") {
                    let tokens = command.split_whitespace().collect::<Vec<&str>>();
                    if tokens.len() > 1 {
                        let rest = &tokens[1..];
                        println!("{}", rest.join(" "));
                    }
                } else {
                    println!("{}: command not found", input.trim());
                }
            }
        }
    }
}
