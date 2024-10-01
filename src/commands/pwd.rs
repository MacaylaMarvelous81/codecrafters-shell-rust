use crate::commands::RunnableCommand;
use crate::ShellState;

pub struct PwdCommand;

impl RunnableCommand for PwdCommand {
    fn exec(&self, state: &mut ShellState, _args: &mut dyn Iterator<Item = &str>) {
        println!("{}", state.directory.display())
    }
}
