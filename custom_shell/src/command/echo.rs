pub fn run(args: Vec<String>) {
    let args = args[1..].join(" ");
    println!("{}", args.trim());
}
