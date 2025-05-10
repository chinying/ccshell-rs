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

        println!("{}: command not found", input.trim());
    }
}
