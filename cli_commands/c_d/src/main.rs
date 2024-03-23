use std::io;
use std::env;
use std::string::String;

fn main() -> io::Result<()> {
    //change current directory.
    
    let args: Vec<String> = env::args().collect(); // grab supplied args as a vec<string>
    
    let mut path = String::from(&args[0]);

   let _ = io::stdin().read_line(&mut path);
    println!("congrats you broke it!");
    Ok(())
}
