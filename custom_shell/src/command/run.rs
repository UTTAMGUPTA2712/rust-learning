use std::process::Command;

pub fn run(args: Vec<String>) {
    if args.len() < 2 {
        println!("run: missing operand");
        return;
    }
    match Command::new("bash").arg("-c").arg(&args[1]).spawn() {
        Ok(mut child) => { child.wait().unwrap(); },
        Err(e) => println!("{e}"),
    }
}
