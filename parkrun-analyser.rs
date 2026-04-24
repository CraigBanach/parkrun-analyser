use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let f = File::open(file_path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
