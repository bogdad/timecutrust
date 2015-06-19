use std::io;
use std::io::prelude::*;

fn main() {
    readlines();
}

fn readlines() {
    let stdin = io::stdin();
    let pattern = "".to_s();
    for r_line in stdin.lock().lines() {
        let line = r_line.unwrap(); 
        if line.matches().count()>0 {
            print!("{}\n", line);
        }
    }
}
