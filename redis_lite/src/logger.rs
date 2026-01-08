use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use chrono::Utc;

pub struct Logger {
    pub file_name: String,
}

impl Logger {
    pub fn write(self: &Self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_file_path = Path::new(&self.file_name);

        let file = OpenOptions::new()
            .append(true) // Open file in append mode
            .create(true) // Create the file if it doesn't exist
            .open(output_file_path)?;

        let mut writer = BufWriter::new(file);

        writeln!(writer, "{}", msg.trim())?;
        writer.flush()?;

        println!(
            "Text appended to {} using 'append' mode.",
            output_file_path.display()
        );
        Ok(())
    }

    pub fn format_message(args: &String) -> String {
        let date_time = Utc::now();
        format!("Log : {} : {:?}", date_time, args)
    }
}
