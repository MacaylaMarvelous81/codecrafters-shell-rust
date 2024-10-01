use crate::commands::RunnableCommand;
use crate::ShellState;

pub struct EchoCommand;

impl RunnableCommand for EchoCommand {
    fn exec(&self, _state: &mut ShellState, args: &mut dyn Iterator<Item = &str>) {
        println!("{}", args.collect::<Vec<&str>>().join(" "));
    }
}
