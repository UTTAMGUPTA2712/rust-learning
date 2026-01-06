use std::process::Command;

pub fn run(args: Vec<&str>) {
    match Command::new("bash").arg(args[1]).spawn() {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}
