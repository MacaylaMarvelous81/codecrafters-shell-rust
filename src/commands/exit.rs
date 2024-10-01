use crate::commands::RunnableCommand;
use crate::ShellState;

pub struct ExitCommand;

impl RunnableCommand for ExitCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item = &str>) {
        state.status = Some(args.next().map(|arg| arg.parse().unwrap_or(0)).unwrap_or(0))
    }
}
