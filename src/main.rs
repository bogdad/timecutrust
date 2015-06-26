extern crate regex;

use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    readlines();
}

fn readlines() {
    let stdin = io::stdin();
    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\]").unwrap();
    for r_line in stdin.lock().lines() {
        let line = r_line.unwrap(); 
        if re.is_match(&line) {
            print!("{}\n", line);
        }
    }
}
