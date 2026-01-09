use custom_grep_tool::{search_buffer, search_case_insensitive_buffer};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);

    let results = if config.ignore_case {
        search_case_insensitive_buffer(&config.query, reader)
    } else {
        search_buffer(&config.query, reader)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
