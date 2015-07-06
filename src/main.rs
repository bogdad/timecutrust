extern crate regex;

use std::cmp::max;
use std::io;
use std::io::{SeekFrom, BufReader, Cursor};
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
fn find_new_line_pos<'a, R: Read + Seek>(reader: &mut BufReader<R>, from: u64) -> Option<usize> { 
    let before = match from.checked_sub(SIZE as u64) {
        None => 0,
        Some(x) => x
    };
    reader.seek(SeekFrom::Start(before)).unwrap();
    let mut buf: [u8;SIZE as usize] = [0; SIZE as usize];
    let len = reader.read(&mut buf).unwrap();
    let last_line = buf[0..len].iter().rposition(|x|*x==0);
    match last_line {
        None => { 
                    let new_before = before.checked_sub(SIZE as u64);
                    match new_before {
                        Some(x) => find_new_line_pos(reader, x),
                        None => None
                    }
                },
        Some(pos) => Some(pos + 1)
    }
}

#[test]
fn find_new_line_pos_works() {
    let data = vec![1,2,0,1,0,5];
    let mut test_data = BufReader::new(Cursor::new(data));
    {
        let pos = find_new_line_pos(&mut test_data, 5);
        assert_eq!(Some(5),pos);
    }
    {
        let pos = find_new_line_pos(&mut test_data, 4);
        assert_eq!(Some(5), pos);
    }
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
