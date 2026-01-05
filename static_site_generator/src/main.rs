use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpServer, web};
use std::{
    env::{self},
    error::Error,
    fmt::format,
    fs::{self, Metadata},
    path::PathBuf,
};

mod convertor;

const STATIC_FOLDER: &str = "static";
const CONTENT_FOLDER: &str = "content";
const TEMPLATE_FOLDER: &str = "templates";

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
    let Commands { dev } = parse_args(&args);

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
        let path = file
            .replace(CONTENT_FOLDER, STATIC_FOLDER)
            .replace(".md", ".html");

        let html = convertor::markdown_to_html(&content);

        fs::write(path, html)?;
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
        let name = match file.file_name().into_string() {
            Ok(v) => v,
            Err(e) => panic!("invalid file exists"),
        };
        let metadata = file.metadata()?;

        if metadata.is_dir() {
            println!("dir");
            let cur_dir = &format!("{}/{}", path, name)[..];
            println!("{cur_dir}");
            let mut ans = flat_map_file_name(cur_dir)?;
            result.append(&mut ans);
        } else if metadata.is_file() {
            println!("file");
            result.push(String::from(format!("{}/{}", path, name).trim()));
        }
    }

    Ok(result)
}
