use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use chrono::Utc;

pub struct Logger {
    writer: BufWriter<File>,
}

impl Logger {
    pub fn new(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let output_file_path = Path::new(file_name);
        
        if let Some(parent) = output_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(output_file_path)?;

        Ok(Logger {
            writer: BufWriter::new(file),
        })
    }

    pub fn write(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.writer, "{}", msg.trim())?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn format_message(args: &str) -> String {
        let date_time = Utc::now();
        format!("Log : {} : {:?}", date_time, args)
    }
}
