extern crate chrono;

use chrono::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::io;

fn main() {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    
    match log_info("log.txt", &formatted) {
        Ok(_) => println!("Time is written to file!"),
        Err(_) => println!("Error: could not create file.")
    }
}

fn log_info(filename: &str, string: &str) -> io::Result<()> {
    let mut f = try!(File::create(filename));
    try!(f.write_all(string.as_bytes()));
    Ok(())
}