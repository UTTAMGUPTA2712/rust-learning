use std::fs;
use std::io::Read;
use std::str;

fn main() {
    println!("PNG Metadata Parser!");

    let file_path = "image.png";
    // let file_path = "rust-tutorial.png";

    let mut file = fs::File::open(file_path).expect("File not found");

    let mut content: Vec<u8> = vec![];
    let png_code: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    file.read_to_end(&mut content).expect("Asd");
    let mut position: usize = 8;

    if content[0..position] != png_code {
        println!("FIle is not a png format");
    }

    if content.len() < 20 {
        println!("Not a valid file");
        return;
    }

    while position < content.len() {
        if position + 8 > content.len() {
             println!("Unexpected end of file reading info");
             break;
        }

        let chunk_length: &[u8] = &content[position..position + 4];

        let bytes: [u8; 4] = chunk_length.try_into().expect("error");
        let length = u32::from_be_bytes(bytes);
        
        let chunk_type: &[u8] = &content[position + 4..position + 8];

        let type_name: &str = std::str::from_utf8(chunk_type).unwrap_or("????");
        println!("Found chunk: {} with length: {}", type_name, length);

        position = position + 12 + (length as usize);
    }
}
