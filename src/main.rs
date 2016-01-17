extern crate regex;
extern crate chrono;

use std::io;
use std::io::{SeekFrom, BufReader, Cursor};
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use regex::Regex;
use std::env;
use binary_search::Predicate;

mod binary_search;
mod regexps;
mod textfileutils;
mod datetimes;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print!("usage timecutrust date_start logfile_name");
    } else {
        work(&args[1], &args[2]);
    }
}

fn work(b: &str, file: &str) -> Result<(), io::Error> {
    let mut f = try!(File::open(file));
    let meta = try!(fs::metadata(file));
    let mut file = BufReader::new(&f);
    let re = datetimes::init();
    let b_time = datetimes::parse(re, b);
    // find the first pos which is after the beg time
    let pred: Predicate = Box::new(|pos: u64| {
        let line = textfileutils::get_first_line_after(file, pos);
        let line_time = datetimes::parse(re, line);
        line_time.timestamp() - b_time.timestamp()
    });
    let start_pos = binary_search::binary_search(0, meta.len(), pred);
    file.seek(SeekFrom::Start(start_pos)).unwrap();

    for r_line in file.lines() {
        let line = r_line.unwrap();
        print!("{}\n", line);
    }
    Ok(())
}

fn readlines() {
    let stdin = io::stdin();
    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\]").unwrap();
    for r_line in stdin.lock().lines() {
        let line = r_line.unwrap(); 
        if regexps::matches(&re, &line) {
            print!("{}\n", line);
        }
    }
}
