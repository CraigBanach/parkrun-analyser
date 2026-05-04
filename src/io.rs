use chrono::NaiveDate;
use std::error::Error;
use std::io::BufReader;
use std::io::Lines;
use std::result::Result;
use std::time::Duration;
use std::{fs::File, io::BufRead};

fn file_lines(file_path: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    Ok(reader.lines())
}

#[derive(Debug)] // TODO: implement Display
pub(crate) struct RunRecord {
    date: NaiveDate,
    event_name: String,
    run_number: u16,
    finish_position: u32,
    time: Duration,
    age_grade: u32,
    personal_best: bool,
}

pub(crate) enum RowOutcome {
    Record(RunRecord),
    Warning(String),
}

pub(crate) fn row_outcomes(
    file_path: &str,
) -> Result<impl Iterator<Item = RowOutcome>, Box<dyn Error>> {
    Ok(file_lines(file_path)?.map(|line| match line {
        Ok(_line) => RowOutcome::Record(RunRecord {
            date: NaiveDate::MIN,
            event_name: "Lochore Meadows".to_string(),
            run_number: 1,
            finish_position: 1,
            time: Duration::MAX,
            age_grade: 10000,
            personal_best: true,
        }),
        Err(e) => RowOutcome::Warning(e.to_string()),
    }))
}
