pub(crate) struct Config {
    file_path: String,
}

impl Config {
    pub(crate) fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }

    pub(crate) fn file_path(&self) -> &str {
        &self.file_path
    }
}
