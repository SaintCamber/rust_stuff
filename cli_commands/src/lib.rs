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

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results : Vec<_> = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    return results

    
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    let mut results : Vec<_> Vec::new();

    for line in contents.lines().to_lowercase(){
        if line.contains(query.to_lowercase()){
            results.push(line)
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

#[cfg(test)]
mod tests {
    use::super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct"
        let contents = "\
        Rust:
        safe,fast,productive,
        Duct tape";

                assert_eq!(vec!["safe, fast, productive."], search(query,contents));

    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
        safe,fast,productive.
        Pick three.
        Trust me";

                assert_eq!(
                    vec!["Rust:","Trust me."],
                    search_case_insensitive(query,contents)
                );
    }

}
