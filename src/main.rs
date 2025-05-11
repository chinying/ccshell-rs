mod os_utils;
mod directory;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

const VALID_COMMANDS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

fn handle_command(command: &str, directory: &mut directory::Directory) {
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
                    let path = os_utils::get_path().unwrap();
                    let dirs = os_utils::get_dir_from_path(&path);
                    let mut found: Option<String> = Option::None;
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
        cmd if cmd.starts_with("cd") => {
            let result = directory.cd(tokens[1]);
            match result {
                Ok(_) => {
                }
                Err(_e) => {
                    let rest = &tokens[1..];
                    eprintln!("cd: {}: No such file or directory", rest.join(" "));
                }
            }
        }
        _ => {
            exec(command);
        }
    }
}

fn exec(command: &str) {
    let tokens = command.split_whitespace().collect::<Vec<&str>>();
    let result = Command::new(tokens[0])
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
    let mut directory = directory::Directory::new();
    while !exit {
        // Wait for user input
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => exit = true,
            command => {
                handle_command(command, &mut directory);
            }
        }
    }
}
