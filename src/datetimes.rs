use regex::Regex;
use chrono::*;

pub fn init() -> Regex {
    let re = Regex::new(r"^\[\(d{4})-(\d{2})-(\d{2})\s(\d{2}):(\d{2}):(\d{2})\]").unwrap();
    re
}

pub fn parse(re: Regex, line: &str) -> DateTime<UTC> {
    let caps = re.captures(line).unwrap();
    let year = caps.at(1).unwrap().parse::<i32>().unwrap();
    let mon = caps.at(2).unwrap().parse::<u32>().unwrap();
    let day = caps.at(3).unwrap().parse::<u32>().unwrap();
    let hh = caps.at(4).unwrap().parse::<u32>().unwrap();
    let mm = caps.at(5).unwrap().parse::<u32>().unwrap();
    let ss = caps.at(6).unwrap().parse::<u32>().unwrap();
    let dt = UTC.ymd(year, mon, day).and_hms(hh, mm, ss);
}