use std::fs;

pub fn run(_: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "./";
    let dir = fs::read_dir(dir_path)?;
    for file in dir {
        let file_name = file?.file_name();
        println!("{}", file_name.as_os_str().display());
    }
    Ok(())
}
