#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut status: Option<u8> = None;

    while status.is_none() {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut args = input.trim_end().split(' ');
        let command = args.next();

        match command {
            Some(command) => {
                match command {
                    "exit" => {
                        status = Some(args.next().map(|arg| {
                            arg.parse().unwrap_or(0)
                        }).unwrap_or(0));
                    }
                    "echo" => {
                        println!("{}", args.collect::<Vec<&str>>().join(" "));
                    }
                    "type" => {
                        let subject = args.next();

                        if subject == Some("exit") || subject == Some("echo") || subject == Some("type") {
                            println!("{} is a shell builtin", subject.unwrap_or(""))
                        } else {
                            println!("{}: not found", subject.unwrap_or(""))
                        }
                    }
                    "" => {}
                    _ => {
                        println!("{}: command not found", command);
                    }
                }
            }
            None => {}
        }
    }
}
