use std::env;
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
            "pwd" => Self::Builtin(Box::new(PwdCommand {})),
            "cd" => Self::Builtin(Box::new(CdCommand {})),
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
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item=&str>);
}

struct ExitCommand;

impl RunnableCommand for ExitCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item=&str>) {
        state.status = Some(args.next().map(|arg| arg.parse().unwrap_or(0)).unwrap_or(0))
    }
}

struct EchoCommand;

impl RunnableCommand for EchoCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item=&str>) {
        println!("{}", args.collect::<Vec<&str>>().join(" "));
    }
}

struct TypeCommand;

impl RunnableCommand for TypeCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item=&str>) {
        let subject = args.next();
        subject.map(|name| {
            let command = ShellCommand::new(name);

            match command {
                ShellCommand::Builtin(_) => println!("{} is a shell builtin", subject.unwrap_or("")),
                ShellCommand::Executable(path) => println!("{} is {}", subject.unwrap_or(""), path.display()),
                ShellCommand::Unknown => println!("{}: not found", subject.unwrap_or(""))
            }
        });
    }
}

struct CdCommand;

impl RunnableCommand for CdCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item=&str>) {
        match args.next() {
            Some(path) => {
                let path = PathBuf::from(path);

                match path.try_exists() {
                    Ok(exists) => {
                        if exists {
                            // BUG: Assumes that the path is absolute (which it might not be!)
                            state.directory = path
                        } else {
                            println!("cd: {}: No such file or directory", path.display())
                        }
                    }
                    Err(err) => println!("Failed to check existence of {}: {}", path.display(), err)
                }
            },
            None => todo!()
        }
    }
}

struct PwdCommand;

impl RunnableCommand for PwdCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item=&str>) {
        println!("{}", state.directory.display())
    }
}

struct ShellState {
    status: Option<u8>,
    directory: PathBuf
}

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut state = ShellState {
        status: None,
        directory: env::current_dir().unwrap_or(env::temp_dir())
    };

    while state.status.is_none() {
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
