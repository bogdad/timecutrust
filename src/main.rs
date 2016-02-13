#![feature(unboxed_closures, fn_traits)]
extern crate regex;
extern crate chrono;
extern crate getopts;

use std::marker::PhantomData;
use std::io::{self, BufReader, Lines, Write};
use std::fs::File;
use std::fs;
use std::io::prelude::{Seek, Read, BufRead};
use std::io::SeekFrom;
use std::env;

use regex::Regex;
use chrono::UTC;
use chrono::{DateTime, Duration};

use getopts::Options;

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

    fn call_inner_mut(&mut self, args: (u64,)) -> i64 {
        let (pos,) = args;

        let mut line_timestamp: i64 = -1;
        loop {
            let line_o = textfileutils::get_first_line_after(& mut self.file, pos);
            if line_o.is_none() {
                break
            }
            let line = line_o.unwrap();
            let line_time_o = datetimes::parse(&self.re, &line);
            if line_time_o.is_some() {
                line_timestamp = line_time_o.unwrap().timestamp();
                break;
            }
        }
        line_timestamp - self.b_time.timestamp()
    }

    fn call_inner(mut self, args: (u64,)) -> i64 {
        self.call_inner_mut(args)
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

fn print_usage(opts: Options) {
    let program = "timecutrust";
    let brief = format!("Usage: {} [options] 'beg-time' file", program);
    print!("{}", opts.usage(&brief));
    println!("examples:");
    println!("timecutrust '[2015-12-28 20:37:25]' ./sample_g2.log");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("r", "regexp", "regular expresrion", "REGULAR_EXPRESSION");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("n", "nginx", "run on nginx access.log with pattern like '[28/Dec/2015:06:26:08'");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    if matches.free.len() < 1 {
        print_usage(opts);
    } else {
        let mut re = matches.opt_str("r")
            .unwrap_or(r"^\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})\]"
                       .to_string());
        if matches.opt_present("n") {
            re = r"^\[(?P<day>\d{2})/(?P<monthname>\p{L}*)/(?P<year>\d{4}):(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})".to_string();
        }
        match work_on_files(&matches.free[0], &matches.free[1], &re) {
            Ok(_) => println!("done."),
            Err(e) => {
                println!("error parsing header: {}", e);
            }
        };
    }
}

fn work_on_files<'a>(b: &'a str, f_name: &'a str, re_str: &'a str) -> Result<(), io::Error> {
    let f = try!(File::open(f_name));
    let meta = try!(fs::metadata(f_name));
    let file = BufReader::new(&f);
    println!("date regexp {}", re_str);
    work_pred(b, f_name, re_str, file, meta.len())
}

fn work_pred<'a, R: 'a + Read + Seek>(b: &'a str, f_name: &'a str, re_s: &'a str, file: BufReader<R>, len: u64) -> Result<(), io::Error> {
    let re = datetimes::init(re_s);
    let b_time = datetimes::parse(&re, b).unwrap() - Duration::milliseconds(1);
    let pred: FilePredicate<R> = FilePredicate::new(file, re, b_time);
    let start_pos = try!(get_start_pos(pred, len));
    work_end(f_name, start_pos)
}

fn get_start_pos<'a, R: 'a + Read + Seek>(mut pred: FilePredicate<'a, R>, len: u64) -> Result<(u64), io::Error> {
    let start_pos = binary_search::binary_search(0, len, & mut pred);
    Ok(start_pos)
}

fn work_end(f_name: &str, start_pos: u64) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let f = try!(File::open(f_name));
    let mut file = BufReader::new(&f);
    file.seek(SeekFrom::Start(start_pos)).unwrap();
    for r_line in file.lines() {
        let line = r_line.unwrap();
        handle.write(line.as_bytes());
        handle.write(b"\n");
    }
    Ok(())
}


