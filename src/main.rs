use std::fs::File;
use std::io::{self, Read};
use std::env;
use std::process::exit;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        println!("Usage: <namefile>");
        exit(1)
    }
    
    let content = read_file(&args[1].as_str())?;

    /*
        pos0 = Tentativi fallitti su admin o root
        pos1 = Tentativi DDoS
        pos2 = Crwaler botnet
        pos3 = Port Scan 
    
    */

    let mut analysis: Vec<i32> = vec![0,0,0,0,0];
    content.lines().for_each(|line| {
        let normalize = line.to_lowercase();
        if normalize.contains("failed password") && normalize.contains("admin") || normalize.contains("root") {
            analysis[0] += 1;
        }else if normalize.contains("firewall") && normalize.contains("ddos alert"){
            analysis[1] += 1;
        }else if normalize.contains("botnet") {
            analysis[2] += 1;
        }else if normalize.contains("kernel") && normalize.contains("port scan") {
            analysis[3] += 1;
        }
    
    });

    println!("
        =============================================
        ||\t\tANALISI COMPLETATA\t   ||
        =============================================

        [*] Tentativi falliti di login su admin o root: {:}
        [*] Tentativi DDoS: {:}
        [*] Tentativi Crawler botnet: {:}
        [*] Tentativi port scan: {:}
    ", analysis[0], analysis[1], analysis[2], analysis[3]);
    Ok(())
}


fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    let _ = File::open(filename)?.read_to_string(&mut content);
    Ok(content)
}