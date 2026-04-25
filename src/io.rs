use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;

pub(crate) fn file_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    Ok(reader.lines())
}
