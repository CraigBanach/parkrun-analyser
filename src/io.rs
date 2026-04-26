use std::error::Error;
use std::io::BufReader;
use std::io::Lines;
use std::result::Result;
use std::{fs::File, io::BufRead};

fn file_lines(file_path: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    Ok(reader.lines())
}

pub(crate) enum RowOutcome {
    Record(String),
    Warning(String),
}

pub(crate) fn row_outcomes(
    file_path: &str,
) -> Result<impl Iterator<Item = RowOutcome>, Box<dyn Error>> {
    Ok(file_lines(file_path)?.map(|line| match line {
        Ok(line) => RowOutcome::Record(line),
        Err(e) => RowOutcome::Warning(e.to_string()),
    }))
}
