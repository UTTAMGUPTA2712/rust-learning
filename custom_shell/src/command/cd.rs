use std::env;
use std::path::Path;

pub fn run(args: Vec<String>) {
    let new_dir = args.get(1).map_or("/", |x| x.as_str());
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("cd: {}: {}", new_dir, e);
    }
}
