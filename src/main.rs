extern crate regex;

use std::cmp::max;
use std::io;
use std::io::{SeekFrom, BufReader};
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;


fn main() {
    readlines();
}

fn matches(re: &Regex, line: &str) -> bool {
    re.is_match(line)
}

fn get_first_line_after(f: &File, from: SeekFrom) {
    let cur = from;
    
}

const SIZE: u32 = 512;

fn find_new_line_pos(f: &File, from:SeekFrom) {
    let mut reader = BufReader::new(f);
    let before = match from {
        SeekFrom::Start(pos) => SeekFrom::Start(pos.checked_sub(SIZE as u64).unwrap()),
        SeekFrom::End(pos) => SeekFrom::End(pos.checked_add(SIZE as i64).unwrap()),
        SeekFrom::Current(pos) => SeekFrom::Current(pos.checked_sub(SIZE as i64).unwrap())
    };
    reader.seek(before);
    let mut buf: [u8;SIZE as usize] = [0; SIZE as usize];
    let nread = reader.read(&mut buf);
    let mut it = buf.enumerate();
    let last_line = it.rposition(|&x,&i| *x == 0); 
}

fn readlines() {
    let stdin = io::stdin();
    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\]").unwrap();
    for r_line in stdin.lock().lines() {
        let line = r_line.unwrap(); 
        if matches(&re, &line) {
            print!("{}\n", line);
        }
    }
}
