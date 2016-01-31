use regex::Regex;
use chrono::*;

pub fn init(r: &str) -> Regex {
    let re = Regex::new(r).unwrap();
    re
}

pub fn parse(re: &Regex, line: &str) -> Option<DateTime<UTC>> {
    let caps = re.captures(line).unwrap();
    let year = caps.at(1).unwrap().parse::<i32>().unwrap();
    let mon = caps.at(2).unwrap().parse::<u32>().unwrap();
    let day = caps.at(3).unwrap().parse::<u32>().unwrap();
    let hh = caps.at(4).unwrap().parse::<u32>().unwrap();
    let mm = caps.at(5).unwrap().parse::<u32>().unwrap();
    let ss = caps.at(6).unwrap().parse::<u32>().unwrap();
    let dt = UTC.ymd(year, mon, day).and_hms(hh, mm, ss);
    Some(dt)
}

#[test]
fn test_sample_regexp() {
    let re = init(r"^\[(\d{4})-(\d{2})-(\d{2})\s(\d{2}):(\d{2}):(\d{2})\]");
    let ti = parse(&re, "[2015-12-28 20:37:25]");
    assert_eq!(2015, ti.unwrap().year());
}
