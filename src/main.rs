extern crate regex;

use std::borrow::Cow;
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

fn get_first_line_after(f: &File, from: u64) -> String {
    let cur = from;
    String::new()
}

const SIZE: usize = 256;
// it is able to find strings up-to 256 elements wide
fn find_new_line_pos<'a, R: Read + Seek>(reader: &mut BufReader<R>, from: usize) -> Option<String> { 
    let before = match from.checked_sub(SIZE) {
        None => 0,
        Some(x) => x
    };
    reader.seek(SeekFrom::Start(before as u64)).unwrap();
    let mut buf: [u8;2*SIZE] = [0; 2*SIZE];
    let len = reader.read(&mut buf).unwrap();
    let last_before = buf[0..len].iter().enumerate().rposition(|(i, x)| *x==b'\n' && (i + before) < from);
    let last_after = buf[0..len].iter().enumerate().position(|(i, x)| *x==b'\n' && (i + before) > from + 1);
    let str_before = match last_before {
        None => "".to_string(),
        Some(pos) => String::from_utf8_lossy(&buf[pos+1..from-before]).into_owned()
    };
    let str_after = match last_after {
        None => "".to_string(),
        Some(pos) => String::from_utf8_lossy(&buf[from-before..pos]).into_owned()
    };
    Some(str_before + &str_after)
}

#[test]
fn find_new_line_pos_works() {
    let data = String::from("some\nother\nline");
    let bytes = data.as_bytes();
    let mut test_data = BufReader::new(Cursor::new(bytes));
    {
        let pos = find_new_line_pos(&mut test_data, 5);
        assert_eq!(Some("other".to_string()),pos);
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
