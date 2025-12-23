use std::fs;
use std::env;

fn main() {
    println!("Grep tool!");
    let args:Vec<String> = env::args().collect();

    if args.len()!=3 {
        println!("Invalid Command format: grep <pattern> <path>");
        return;
    }

    let pattern = &args[1];
    let path = &args[2];
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    println!("Searching for '{}' in '{}'", pattern, path);

    if fs::metadata(path).is_err() {
        println!("Error: File '{}' does not exist.", path);
        return;
    }

    let file_content = fs::read_to_string(path).expect("File Does Not Exists");

    for file_line in file_content.lines() {
        if file_line.contains(pattern){
            println!("{}",file_line);
        }
    }
}
