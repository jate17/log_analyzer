use std::fs::File;
use std::io::{self, Read};



fn main() -> io::Result<()> {
    let mut content = String::new();
    File::open("log.txt")?.read_to_string(&mut content);
    println!("{}", content);
    Ok(())
}
