pub fn run(args: Vec<&str>) {
    let args = args[1..].join(" ");
    println!("{:?}", args.trim());
}
