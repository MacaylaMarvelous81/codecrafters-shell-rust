use std::env::current_dir;
use crate::commands::RunnableCommand;
use crate::ShellState;

pub struct PwdCommand;

impl RunnableCommand for PwdCommand {
    fn exec(&self, _state: &mut ShellState, _args: &mut dyn Iterator<Item = &str>) {
        match current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(err) => println!("Could not access current directory: {}", err)
        }
    }
}
