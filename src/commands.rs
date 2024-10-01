use crate::commands::{
    cd::CdCommand, echo::EchoCommand, exit::ExitCommand, pwd::PwdCommand, r#type::TypeCommand,
};
use crate::ShellState;
use std::path::PathBuf;
use which::which;

mod cd;
mod echo;
mod exit;
mod pwd;
mod r#type;

pub trait RunnableCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item = &str>);
}

pub enum ShellCommand {
    Builtin(Box<dyn RunnableCommand>),
    Executable(PathBuf),
    Unknown,
}

impl ShellCommand {
    pub fn new(command: &str) -> Self {
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
