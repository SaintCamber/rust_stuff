use std::io;
use std::env;
use std::string::String;

fn main() -> io::Result<()> {
    //change current directory.
    
    let args: Vec<String> = env::args().collect(); // grab supplied args as a vec<string>
    
    let mut path = String::from(&args[1]);

    println!("congrats you broke it!");
    println!("{}",path);
    let stdin = io::stdin;
    let mut handle =stdin.lock();
    Ok(())
}
