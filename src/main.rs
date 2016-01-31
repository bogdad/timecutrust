#![feature(unboxed_closures, fn_traits)]
extern crate regex;
extern crate chrono;

use std::marker::PhantomData;
use std::io;
use std::io::{BufReader, Lines};
use std::fs::File;
use std::fs;
use std::io::prelude::{Seek, Read, BufRead};
use regex::Regex;
use std::env;

use chrono::UTC;
use chrono::DateTime;

mod binary_search;
mod regexps;
mod textfileutils;
mod datetimes;



struct FilePredicate<'a, R: 'a + Read + Seek> {
    file: BufReader<R>,
    re: regex::Regex,
    b_time: DateTime<UTC>,
    phantom: PhantomData<&'a BufReader<R>>
}

impl<'a, R: 'a + Read + Seek> FilePredicate<'a, R> {
    fn new(file: BufReader<R>, re: Regex, b_time: DateTime<UTC>) -> FilePredicate<'a, R> {
        FilePredicate{ file: file, re: re, b_time: b_time, phantom: PhantomData }
    }

    fn lines(self) -> Lines<BufReader<R>> {
        self.file.lines()
    }

    fn call_inner_mut(&mut self, args: (u64,)) -> i64 {
        let (pos,) = args;
        let line = textfileutils::get_first_line_after(& mut self.file, pos);
        let line_time = datetimes::parse(&self.re, &line);
        line_time.timestamp() - self.b_time.timestamp()
    }

    fn call_inner(mut self, args: (u64,)) -> i64 {
        let (pos,) = args;
        let line = textfileutils::get_first_line_after(& mut self.file, pos);
        let line_time = datetimes::parse(&self.re, &line);
        line_time.timestamp() - self.b_time.timestamp()
    }
}

impl<'a, R: 'a + Read + Seek> FnOnce<(u64,)> for FilePredicate<'a, R> {
    type Output = i64;
    extern "rust-call" fn call_once(self, pos: (u64,)) -> i64 {
        self.call_inner(pos)
    }
}

impl<'a, R: 'a + Read + Seek> FnMut<(u64,)> for FilePredicate<'a, R> {
    extern "rust-call" fn call_mut(&mut self, pos: (u64,)) -> i64 {
        self.call_inner_mut(pos)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage timecutrust date_start logfile_name");
    } else {
        match work_on_files(&args[0], &args[1]) {
            Ok(_) => println!("done."),
            Err(e) => {
                println!("error parsing header: {}", e);
            }
        };
    }
}

fn work_on_files<'a>(b: &'a str, f_name: &'a str) -> Result<(), io::Error> {
    let f = try!(File::open(f_name));
    let meta = try!(fs::metadata(f_name));
    let file = BufReader::new(&f);
    work_pred(b, file, meta.len())
}

fn work_pred<'a, R: 'a + Read + Seek>(b: &'a str, file: BufReader<R>, len: u64) -> Result<(), io::Error> {
    let re = datetimes::init();
    let b_time = datetimes::parse(&re, b);
    let pred: FilePredicate<R> = FilePredicate::new(file, re, b_time);
    work(pred, len)
}

fn work<'a, R: 'a + Read + Seek>(mut pred: FilePredicate<'a, R>, len: u64) -> Result<(), io::Error> {
    binary_search::binary_search(0, len, & mut pred);
    for r_line in pred.lines() {
        let line = r_line.unwrap();
        print!("{}\n", line);
    }
    Ok(())
}

/*
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
*/
