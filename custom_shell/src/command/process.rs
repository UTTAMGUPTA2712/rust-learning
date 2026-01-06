use std::process::{Command, Stdio};

pub fn run(mut args: Vec<&str>) {
    let echo_child = Command::new("echo")
        .arg("Oh no, a tpyo!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");

    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

    args.insert(0, "ubuntu@ip");
    args.insert(0, "/tmp/.ssh/25.pem");
    args.insert(0, "-i");

    match Command::new("use/bin/ssh")
        .args(args)
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}
