#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let will_terminate = false;

    while !will_terminate {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut args = input.trim_end().split(' ');
        let command = args.next();

        match command {
            Some(command) => {
                if command != "" {
                    println!("{}: command not found", command);
                }
            }
            None => {}
        }
    }
}
