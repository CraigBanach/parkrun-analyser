use crate::config::Config;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub(crate) fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let f = File::open(config.file_path())?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
