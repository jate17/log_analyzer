use std::fs::{File, read};
use std::io::{self, Read};
use std::env;
use std::process::exit;
use std::task::Context;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        println!("Usage: <namefile>");
        exit(1)
    }
    
    let content = read_file(&args[1].as_str())?;
    content.lines().for_each(|line| println!("{}", line));
    
    Ok(())
}


fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    File::open("log.txt")?.read_to_string(&mut content);
    Ok(content)
}