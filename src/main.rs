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

type Predicate<'a> = &'a(Fn(usize) -> usize);

fn binary_search(len: usize, predicate: Predicate) -> Option<usize> {

     let mut beg = 0;
     let mut end = len;
     binary_search_inner(predicate, beg, end)
}

#[test]
fn test_binary_search() {
    let pred = &(|i:usize| {i-5});
    let res = binary_search(10, pred);
    assert_eq!(res, Some(5))
}

fn binary_search_inner(
        predicate: Predicate, pi_beg: usize, pi_end: usize) -> Option<usize> {
    let mut i_beg = pi_beg;
    let mut i_end = pi_end;
    let beg = predicate(i_beg);
    if (beg == 0) {
        return Some(i_beg);
    }
    let end = predicate(i_end);
    if (end == 0) {
        return Some(i_end);
    }
    while (i_end - i_beg <= 1) {
        let mid = i_beg + (i_end-i_beg)/2;
        let pval = predicate(mid);
        if (pval == 0) {
            return Some(mid);
        } else if (pval>0) {
            i_beg = mid;
        } else {
            i_end = mid;
        }
    }
    let x: Option<usize> = None;
    x
}

fn get_first_line_after<'a, R: Read + Seek>(reader: &mut BufReader<R>, from: usize) -> String {
    find_new_line_pos(reader, from).unwrap()
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
    let bufs = &buf[0..len];
    print!("{:?}", String::from_utf8_lossy(bufs));
    let last_before = bufs.iter().enumerate().rposition(|(i, x)| *x==b'\n' && (i + before) < from);
    let last_after = bufs.iter().enumerate().position(|(i, x)| *x==b'\n' && (i + before) >= from);
    let str_before = match last_before {
        None => String::from_utf8_lossy(&buf[0..from-before]).into_owned(),
        Some(pos) => String::from_utf8_lossy(&buf[pos+1..from-before]).into_owned()
    };
    let str_after = match last_after {
        None => String::from_utf8_lossy(&buf[from-before..len]).into_owned(),
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
    {
        let pos = find_new_line_pos(&mut test_data, 1);
        assert_eq!(Some("some".to_string()), pos)
    }
    {
        let pos = find_new_line_pos(&mut test_data, 4);
        assert_eq!(Some("some".to_string()), pos);
    }
    {
        let pos = find_new_line_pos(&mut test_data, 10);
        assert_eq!(Some("other".to_string()), pos);
    }
    {
        let pos = find_new_line_pos(&mut test_data, 11);
        assert_eq!(Some("line".to_string()), pos);
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
