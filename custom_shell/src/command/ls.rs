use std::fs;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let default = ".".to_string();
    let dir_path = args.get(1).unwrap_or(&default);
    
    match fs::read_dir(dir_path) {
        Ok(dir) => {
            for file in dir {
                if let Ok(file) = file {
                    let file_name = file.file_name();
                    println!("{}", file_name.to_string_lossy());
                }
            }
        },
        Err(e) => eprintln!("ls: cannot access '{}': {}", dir_path, e),
    }
    Ok(())
}
