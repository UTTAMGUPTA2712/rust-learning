use std::{
    env,
    io::{self, Write},
};

mod command;

fn main() {
    let env_args: Vec<String> = env::args().collect();

    let linux = env_args.get(1).map(|arg| arg == "--local").unwrap_or(false);
    
    if linux {
        println!("----------------Using Linux Commands----------------");
    } else {
        println!("----------------Using Local Commands----------------");
    }
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let args: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

        if args.is_empty() {
            continue;
        }

        let command = args[0].to_lowercase();

        if linux {
            command::process::run(args);
        } else {
            match command.as_str() {
                "ls" => {
                    let _ = command::ls::run(args);
                }
                "cd" => {
                    command::cd::run(args);
                }
                "run" => {
                    command::run::run(args);
                }
                "echo" => {
                    command::echo::run(args);
                }
                "exit" => return,
                _ => println!("{}: command not found", command),
            }
        }
    }
}
