use std::env::set_current_dir;
use crate::commands::RunnableCommand;
use crate::ShellState;
use std::path::PathBuf;

pub struct CdCommand;

impl RunnableCommand for CdCommand {
    fn exec(&self, _state: &mut ShellState, args: &mut dyn Iterator<Item = &str>) {
        match args.next() {
            Some(path) => {
                let path = PathBuf::from(path);

                match path.try_exists() {
                    Ok(exists) => {
                        if exists {
                            if let Err(err) = set_current_dir(path) {
                                println!("Failed to change directory: {}", err);
                            }
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
