use std::error::Error;

use crate::config::Config;
use crate::io::file_lines;

pub(crate) fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    for line in file_lines(config.file_path())? {
        println!("{}", line?);
    }
    Ok(())
}
