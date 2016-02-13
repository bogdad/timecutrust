use regex::Regex;
use chrono::*;

pub fn init(r: &str) -> Regex {
    let re = Regex::new(r).unwrap();
    re
}

pub fn parse(re: &Regex, line: &str) -> Option<DateTime<UTC>> {
    let caps = re.captures(line).unwrap();
    let year = caps.name("year").unwrap().parse::<i32>().unwrap();
    let mon = caps.name("month").unwrap().parse::<u32>().unwrap();
    let day = caps.name("day").unwrap().parse::<u32>().unwrap();
    let hh = caps.name("hour").unwrap().parse::<u32>().unwrap();
    let mm = caps.name("minute").unwrap().parse::<u32>().unwrap();
    let ss = caps.name("second").unwrap().parse::<u32>().unwrap();
    let dt = UTC.ymd(year, mon, day).and_hms(hh, mm, ss);
    Some(dt)
}

#[test]
fn test_sample_regexp() {
    let re = init(r"^\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})\]");
    let ti = parse(&re, "[2015-12-28 20:37:25]");
    assert_eq!(2015, ti.unwrap().year());
}
