use crate::commands::{RunnableCommand, ShellCommand};
use crate::ShellState;

pub struct TypeCommand;

impl RunnableCommand for TypeCommand {
    fn exec(&self, _state: &mut ShellState, args: &mut dyn Iterator<Item = &str>) {
        let subject = args.next();

        if let Some(name) = subject {
            let command = ShellCommand::new(name);

            match command {
                ShellCommand::Builtin(_) => {
                    println!("{} is a shell builtin", subject.unwrap_or(""))
                }
                ShellCommand::Executable(path) => {
                    println!("{} is {}", subject.unwrap_or(""), path.display())
                }
                ShellCommand::Unknown => println!("{}: not found", subject.unwrap_or("")),
            }
        }
    }
}
