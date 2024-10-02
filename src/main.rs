mod commands;

use crate::commands::ShellCommand;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

struct ShellState {
    status: Option<u8>
}

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut state = ShellState {
        status: None
    };

    while state.status.is_none() {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut args = input.trim_end().split(' ');
        let command_name = args.next();

        if let Some(command_name) = command_name {
            let command = ShellCommand::new(command_name);

            match command {
                ShellCommand::Builtin(command) => {
                    command.exec(&mut state, &mut args);
                }
                ShellCommand::Executable(path) => {
                    let result = Command::new(path)
                        .args(args)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .output();

                    if result.is_err() {
                        println!("failed to execute {}", command_name);
                    }
                }
                ShellCommand::Unknown => {
                    if !command_name.is_empty() {
                        println!("{}: command not found", command_name);
                    }
                }
            }
        }
    }
}
