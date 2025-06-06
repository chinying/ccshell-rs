mod directory;
mod os_utils;
mod parser;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

const VALID_COMMANDS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

fn handle_command(command: &str, directory: &mut directory::Directory) -> io::Result<()> {
    let parts: Vec<String> = parser::parse_command(command);
    let tokens = parts.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    if tokens.is_empty() {
        return Ok(());
    }
    match tokens[0] {
        "echo" => {
            let rest = &tokens[1..];
            println!("{}", rest.join(" "));
        }
        "type" => {
            if tokens.len() > 1 {
                if VALID_COMMANDS.contains(&tokens[1]) {
                    println!("{} is a shell builtin", tokens[1]);
                } else {
                    let path = os_utils::get_path().unwrap();
                    let dirs = os_utils::get_dir_from_path(&path);
                    let mut found: Option<String> = None;
                    for dir in dirs {
                        // macos things https://apple.stackexchange.com/q/458277
                        if dir.contains("com.apple.security.cryptexd") {
                            continue;
                        }
                        if os_utils::list_dir(&dir)
                            .unwrap()
                            .contains(&tokens[1].to_string())
                        {
                            found = Some(dir);
                            break;
                        }
                    }
                    match found {
                        Some(dir) => {
                            println!("{} is {}/{}", tokens[1], &dir, tokens[1]);
                        }
                        None => {
                            println!("{}: not found", tokens[1]);
                        }
                    }
                }
            }
        }
        "pwd" => {
            println!("{}", directory.pwd());
        }
        "cd" => {
            if tokens.len() > 1 {
                directory.cd(tokens[1])?;
            } else {
                eprintln!("cd: too few arguments");
            }
        }
        _ => {
            exec(command);
        }
    }
    Ok(())
}

fn exec(command: &str) {
    let tokens = parser::parse_command(command);
    let result = Command::new(tokens[0].clone())
        .args(&tokens[1..])
        .spawn()
        .and_then(|mut child| child.wait());

    match result {
        Ok(status) => {
            if !status.success() {
                eprintln!("{}: command not found", command);
            }
        }
        Err(_e) => {
            eprintln!("{}: command not found", command);
        }
    }
}

fn main() {
    // by default, treat everything as invalid
    let mut exit = false;
    let mut directory = directory::Directory::new().unwrap(); // TODO: handle error later
    while !exit {
        // Wait for user input
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => exit = true,
            command => {
                if let Err(e) = handle_command(command, &mut directory) {
                    eprintln!("{}", e);
                }
            }
        }
    }
}
