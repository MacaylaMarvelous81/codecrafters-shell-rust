use which::which;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

enum ShellCommand {
    Builtin(Box<dyn RunnableCommand>),
    Executable(PathBuf),
    Unknown
}

impl ShellCommand {
    fn new(command: &str) -> Self {
        match command {
            "exit" => Self::Builtin(Box::new(ExitCommand {})),
            "echo" => Self::Builtin(Box::new(EchoCommand {})),
            "type" => Self::Builtin(Box::new(TypeCommand {})),
            "" => Self::Unknown,
            name => {
                let exec_path = which(name);
                if let Ok(exec_path) = exec_path {
                    Self::Executable(exec_path)
                } else {
                    Self::Unknown
                }
            }
        }
    }
}

trait RunnableCommand {
    fn exec(&self, args: &mut dyn Iterator<Item=&str>) -> Option<u8>;
}

struct ExitCommand {}

impl RunnableCommand for ExitCommand {
    fn exec(&self, args: &mut dyn Iterator<Item=&str>) -> Option<u8> {
        Some(args.next().map(|arg| arg.parse().unwrap_or(0)).unwrap_or(0))
    }
}

struct EchoCommand {}

impl RunnableCommand for EchoCommand {
    fn exec(&self, args: &mut dyn Iterator<Item=&str>) -> Option<u8> {
        println!("{}", args.collect::<Vec<&str>>().join(" "));

        None
    }
}

struct TypeCommand {}

impl RunnableCommand for TypeCommand {
    fn exec(&self, args: &mut dyn Iterator<Item=&str>) -> Option<u8> {
        let subject = args.next();
        subject.map(|name| {
            let command = ShellCommand::new(name);

            match command {
                ShellCommand::Builtin(_) => println!("{} is a shell builtin", subject.unwrap_or("")),
                ShellCommand::Executable(path) => println!("{} is {}", subject.unwrap_or(""), path.display()),
                ShellCommand::Unknown => println!("{}: not found", subject.unwrap_or(""))
            }
        });

        None
    }
}

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut status: Option<u8> = None;

    while status.is_none() {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut args = input.trim_end().split(' ');
        let command_name = args.next();

        command_name.map(|command_name| {
            let command = ShellCommand::new(command_name);

            match command {
                ShellCommand::Builtin(command) => {
                    status = command.exec(&mut args);
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
                },
                ShellCommand::Unknown => {
                    if command_name != "" {
                        println!("{}: command not found", command_name);
                    }
                }
            }
        });
    }
}
