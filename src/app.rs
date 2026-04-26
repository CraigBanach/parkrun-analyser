use std::error::Error;

use crate::config::Config;
use crate::io::{RowOutcome, row_outcomes};

pub(crate) fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    for outcome in row_outcomes(config.file_path())? {
        match outcome {
            RowOutcome::Record(line) => println!("{}", line),
            RowOutcome::Warning(msg) => println!("warning: {}", msg),
        }
    }
    Ok(())
}
