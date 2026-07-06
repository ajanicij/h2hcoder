use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};

fn main() {
    match run() {
        Ok(()) => (),
        Err(msg) => {
            eprintln!("Error: {}", msg);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("no file name").into());
    }
    let file_name = &args[1];
    println!("File name: {}", file_name);
    let file = File::open(file_name)?;
    println!("Opened file {}", file_name);
    let reader = io::BufReader::new(file);
    let mut count = reader.lines().count();

/*    
    for _line in reader.lines() {
        count += 1;
    }
*/
    println!("Count: {}", count);
    Ok(())
}
