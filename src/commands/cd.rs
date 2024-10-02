use crate::commands::RunnableCommand;
use crate::ShellState;
use std::env::{set_current_dir, var_os};
use std::path::{Component, PathBuf};

pub struct CdCommand;

impl RunnableCommand for CdCommand {
    fn exec(&self, _state: &mut ShellState, args: &mut dyn Iterator<Item = &str>) {
        match args.next() {
            Some(path) => {
                let input_path = PathBuf::from(path);
                let mut path = PathBuf::new();

                for component in input_path.components() {
                    if let Component::Normal(segment) = component {
                        if segment == "~" {
                            path.push(var_os("HOME").unwrap_or_default());
                        } else {
                            path.push(component);
                        }
                    } else {
                        path.push(component);
                    }
                }

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
