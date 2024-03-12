use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config){
        println!("Application error: {e}");
        process::exit(1);
    }

}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
fn build(args: &[String]) -> Result <Config, &'static str> {
    if args.len() < 3 {
        let len = args.len();
        panic!("not enough arguments: {len} were supplied but 3 are required")
    }
    let query = &args[1];
    let file_path = &args[2];

    Ok (Config {query: query.to_string(),file_path: file_path.to_string()})
}
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}