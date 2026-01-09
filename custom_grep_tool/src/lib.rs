use std::io::BufRead;

pub fn search_buffer<R: BufRead>(query: &str, reader: R) -> Vec<String> {
    reader
        .lines()
        .map(|l| l.unwrap_or_default())
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive_buffer<R: BufRead>(query: &str, reader: R) -> Vec<String> {
    let query = query.to_lowercase();
    reader
        .lines()
        .map(|l| l.unwrap_or_default())
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
