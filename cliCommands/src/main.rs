use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

    

println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        let len = args.len();
        panic!("not enough arguments: {len} were supplied but 3 are required")
    }
    let query = &args[1];
    let file_path = &args[2];

    Config {query: query.to_string(),file_path: file_path.to_string()}
}
}