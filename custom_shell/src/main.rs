use std::{
    env,
    io::{self, Write},
};

mod command;

fn main() {
    let env_args: Vec<String> = env::args().collect();

    let linux = env_args[0] == "--local";
    if linux {
        println!("----------------Using Linux Commands----------------");
    } else {
        println!("----------------Using Local Commands----------------");
    }
    loop {
        let mut args = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut args)
            .expect("Failed to read line");

        let args: Vec<&str> = args.split(" ").collect();

        let command = &args[0].trim().to_lowercase()[..];

        if linux {
            command::process::run(args);
        } else {
            match command {
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
                "" => (),
                _ => println!("{}: command not found", args[0].trim()),
            }
        }
    }
}
