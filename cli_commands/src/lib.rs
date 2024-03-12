use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
pub fn build(args: &[String]) -> Result <Config, &'static str> {
    if args.len() < 3 {
        let len = args.len();
        panic!("not enough arguments: {len} were supplied but 3 are required")
    }
    let query = &args[1];
    let file_path = &args[2];

    Ok (Config {query: query.to_string(),file_path: file_path.to_string()})
}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}