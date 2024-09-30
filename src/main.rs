#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    Builtin(Box<dyn RunnableCommand>),
    Executable,
    Unknown
}

impl Command {
    fn new(command: &str) -> Self {
        match command {
            "exit" => Self::Builtin(Box::new(ExitCommand {})),
            "echo" => Self::Builtin(Box::new(EchoCommand {})),
            "type" => Self::Builtin(Box::new(TypeCommand {})),
            _ => Self::Unknown
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

        if subject == Some("exit") || subject == Some("echo") || subject == Some("type") {
            println!("{} is a shell builtin", subject.unwrap_or(""))
        } else {
            println!("{}: not found", subject.unwrap_or(""))
        }

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
            let command = Command::new(command_name);

            match command {
                Command::Builtin(command) => {
                    status = command.exec(&mut args);
                }
                Command::Executable => {}
                Command::Unknown => {
                    if command_name != "" {
                        println!("{}: command not found", command_name);
                    }
                }
            }
        });
    }
}
