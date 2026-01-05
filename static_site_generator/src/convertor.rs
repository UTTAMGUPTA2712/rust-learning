use crate::convertor::serde_json::Error;
use core::panic;
use pulldown_cmark::{Options, Parser};
use serde_json::{self, json};
use std::fs;
use toml::Table;

pub fn convert_md_to_json(path: &str) -> serde_json::Value {
    let content = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(e) => {
            println!("There is an error opening file with path {}. {}", path, e);
            return json!({});
        }
    };
    match self::convert_to_json(content) {
        Ok(v) => v,
        Err(e) => panic!("Error{}", e),
    }
}

pub fn convert_to_json(content: String) -> Result<serde_json::Value, Error> {
    let json_data: serde_json::Value = json!({});

    let key_value: Vec<&str> = content.split("+++").collect();

    if key_value.len() > 3 {
        return Ok(json!({content:"Invalid Format"}));
    }

    let data = match key_value[1].parse::<Table>() {
        Ok(v) => v,
        Err(e) => panic!("Error {}", e),
    };

    let data = data.into_iter().map(|t| {
        // let pointer = json_data.pointer_mut(&t.0);
        // pointer= &t.1;
    });

    Ok(json_data)
}

pub fn markdown_to_html(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
