use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpServer, web};
use std::{
    env::{self},
    fs::{self},
    path::{Path, PathBuf},
};

mod convertor;

const STATIC_FOLDER: &str = "static";
const CONTENT_FOLDER: &str = "content";

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

struct Commands {
    dev: bool,
}

fn parse_args(args: &[String]) -> Commands {
    let dev = args.iter().find(|x| *x == "--dev").is_some();
    return Commands { dev };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let Commands { dev: _dev } = parse_args(&args);

    let files = match flat_map_file_name(CONTENT_FOLDER) {
        Ok(v) => v,
        Err(e) => panic!("error {}", e),
    };

    for file in files {
        let content = match fs::read_to_string(&file) {
            Ok(value) => value,
            Err(e) => {
                panic!("There is an error opening file. {}", e);
            }
        };

        let source_path = Path::new(&file);
        let relative_path = source_path.strip_prefix(CONTENT_FOLDER).unwrap();
        let target_path = Path::new(STATIC_FOLDER)
            .join(relative_path)
            .with_extension("html");

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let html = convertor::markdown_to_html(&content);

        fs::write(target_path, html)?;
    }

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn flat_map_file_name(path: &str) -> Result<Vec<String>, std::io::Error> {
    let files = fs::read_dir(path)?;

    let mut result: Vec<String> = vec![];
    for file in files {
        let file = file?;
        let path_obj = file.path();

        if path_obj.is_dir() {
            println!("dir");
            let cur_dir = path_obj.to_str().unwrap();
            println!("{}", cur_dir);
            let mut ans = flat_map_file_name(cur_dir)?;
            result.append(&mut ans);
        } else if path_obj.is_file() {
            println!("file");
            if let Some(s) = path_obj.to_str() {
                result.push(String::from(s));
            }
        }
    }

    Ok(result)
}
