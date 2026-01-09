use std::env;
use std::process::{Command, Stdio};

pub fn run(mut args: Vec<String>) {
    let echo_child = Command::new("echo")
        .arg("Oh no, a tpyo!") // Intentionally left as 'tpyo' for the joke/example
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");

    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

    // Use environment variables or defaults, don't hardcode specific IPs/Keys
    let ssh_key = env::var("SSH_KEY").unwrap_or_else(|_| "/tmp/.ssh/id_rsa".to_string());
    let ssh_host = env::var("SSH_HOST").unwrap_or_else(|_| "user@localhost".to_string());

    // Insert in reverse order to maintain position at start
    args.insert(0, ssh_host);
    args.insert(0, ssh_key);
    args.insert(0, "-i".to_string());

    match Command::new("/usr/bin/ssh")
        .args(args)
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(_) => (),
        Err(e) => println!("SSH command failed: {e}"),
    }
}
