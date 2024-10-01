use crate::commands::RunnableCommand;
use crate::ShellState;
use std::path::PathBuf;

pub struct CdCommand;

impl RunnableCommand for CdCommand {
    fn exec(&self, state: &mut ShellState, args: &mut dyn Iterator<Item = &str>) {
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
                    Err(err) => {
                        println!("Failed to check existence of {}: {}", path.display(), err)
                    }
                }
            }
            None => todo!(),
        }
    }
}
