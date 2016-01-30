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
        match work_on_files(&args[0], &args[1]) {
            Ok(_) => println!("done."),
            Err(e) => {
                println!("error parsing header: {}", e);
            }
        };
    }
}

fn work_on_files(b: &str, f_name: &str) -> Result<(), io::Error> {
    let f = try!(File::open(f_name));
    let meta = try!(fs::metadata(f_name));
    let file = BufReader::new(&f);
    work(b, &mut file, meta.len());
    Ok(())
}

fn work<'a, R: Read + Seek>(b: &str, file: &mut BufReader<R>, len: u64) -> Result<(), io::Error> {
    let re = datetimes::init();
    let b_time = datetimes::parse(re, b);
    // find the first pos which is after the beg time
    let pred: Predicate = Box::new(|pos: u64| {
        let line = textfileutils::get_first_line_after(&mut file, pos);
        let line_time = datetimes::parse(re, &line);
        line_time.timestamp() - b_time.timestamp()
    });
    let start_pos: u64 = binary_search::binary_search(0, len, pred);
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
