#![feature(core, unboxed_closures)]
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

use chrono::UTC;
use chrono::DateTime;

mod binary_search;
mod regexps;
mod textfileutils;
mod datetimes;



struct FilePredicate<'fp, R: 'fp + Read + Seek> {
    file: &'fp mut BufReader<R>,
    re: regex::Regex,
    b_time: DateTime<UTC>
}

impl<'fp, R: 'fp + Read + Seek> FilePredicate<'fp, R> {
    fn new(file: &'fp mut BufReader<R>, re: Regex, b_time: DateTime<UTC>) -> FilePredicate<'fp, R> {
        FilePredicate{ file: file, re: re, b_time: b_time }
    }

    fn call_inner(&self, pos: u64) -> i64 {
        let line = textfileutils::get_first_line_after(self.file, pos);
        let line_time = datetimes::parse(self.re, &line);
        line_time.timestamp() - self.b_time.timestamp()
    }
}

impl<'fp, R: Read + Seek> FnOnce<(u64)> for FilePredicate<'fp, R> {
    type Output = i64;
    extern "rust-call" fn call_once(self, pos: u64) -> i64 {
        self.call_inner(pos)
    }
}

impl<'fp, R: Read + Seek> FnMut<(u64)> for FilePredicate<'fp, R> {
    extern "rust-call" fn call_mut(&mut self, pos: u64) -> i64 {
        self.call_inner(pos)
    }
}

impl<'fp, R: Read + Seek> Fn<(u64)> for FilePredicate<'fp, R> {
    extern "rust-call" fn call(&self, pos: u64) -> i64 {
        self.call_inner(pos)
    }
}

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

fn work<'a, R: Read + Seek>(b: &'a str, file: &'a mut BufReader<R>, len: u64) -> Result<(), io::Error> {
    let re = datetimes::init();
    let b_time = datetimes::parse(re, b);
    let pred: FilePredicate<'a, R> = FilePredicate::new(file, re, b_time);

    let start_pos: u64 = binary_search::binary_search(0, len, &pred);
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
