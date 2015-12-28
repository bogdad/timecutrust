extern crate regex;

use std::cmp::max;
use std::io;
use std::io::{SeekFrom, BufReader, Cursor};
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    work(args[1].clone());
}

fn matches(re: &Regex, line: &str) -> bool {
    re.is_match(line)
}

type Pred = (Box<Fn(i64) -> i64>);
type Predicate = Pred;

fn binary_search(beg: i64, end: i64, predicate: Predicate) -> i64 {
     binary_search_inner(predicate, beg, end)
}

fn predfactory (item:i64) -> Predicate {
    let pred: Pred =
        Box::new(move |i:i64| { let z:i64 = item.checked_add(i).unwrap(); z});
    pred
}


#[test]
fn test_binary_search() {
    let pred: Predicate = Box::new(|i| { let g:i64 = -5; let z:i64 = g.checked_add(i).unwrap(); z });
    let res = binary_search(0, 10, pred);
    assert_eq!(res, 5);
    assert_eq!(binary_search(0, 9, predfactory(-2)), 2);
    assert_eq!(binary_search(0, 9, predfactory(-9)), 9);
    assert_eq!(binary_search(0, 9, predfactory(-10)), -11);
    assert_eq!(binary_search(0, 9, predfactory(1)), -1);
}

//  1   2   3  4  5 6 7 8 9 10
//  -4 -3  -2 -1  0 1 2 3 4 5
fn binary_search_inner(
        predicate: Predicate, pi_beg: i64, pi_end: i64) -> i64 {
    let mut i_beg = pi_beg;
    let mut i_end = pi_end;
    while (i_beg <= i_end) {
        let mid = i_beg + (i_end-i_beg)/2;
        let pval = predicate(mid);
        print!("{:?} {:?} {:?} {:?} \n", i_beg, i_end, mid, pval);
        if pval == 0 {
            return  mid;
        } else if pval < 0 {
            i_beg = mid + 1;
        } else {
            i_end = mid - 1;
        }
    };
    -(i_beg + 1)
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

#[test]
fn test_sample_regexp() {
    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\]").unwrap();
    assert_eq!(true, matches(&re, "[2015-12-28 20:37:25] @30262 INFO: Processing by AgentController#tasks as"));
}

fn work(file: String) -> Result<(), io::Error> {
    let mut f = try!(File::open(file));
    let mut file = BufReader::new(&f);
    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\]").unwrap();
    for r_line in file.lines() {
        let line = r_line.unwrap();
        if matches(&re, &line) {
            print!("{}\n", line);
        }
    }
    Ok(())
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
