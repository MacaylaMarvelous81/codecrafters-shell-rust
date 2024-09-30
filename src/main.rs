use std::collections::HashMap;

#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut commands: HashMap<&str, fn(&mut dyn Iterator<Item=&str>) -> Option<u8>> = HashMap::new();
    commands.insert("exit", |args| {
        Some(args.next().map(|arg| {
            arg.parse().unwrap_or(0)
        }).unwrap_or(0))
    });
    commands.insert("echo", |args| {
        println!("{}", args.collect::<Vec<&str>>().join(" "));
        None
    });
    commands.insert("type", |args| {
        let subject = args.next();

        if subject == Some("exit") || subject == Some("echo") || subject == Some("type") {
            println!("{} is a shell builtin", subject.unwrap_or(""))
        } else {
            println!("{}: not found", subject.unwrap_or(""))
        }
        None
    });

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
                status = match commands.get(command) {
                    Some(func) => func(&mut args),
                    None => {
                        if command != "" {
                            println!("{}: command not found", command);
                        }
                        None
                    }
                };
            }
            None => {}
        }
    }
}
