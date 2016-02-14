use regex::Regex;
use chrono::format::Fixed;
use chrono::format::Item;
use chrono::format::parsed::*;
use chrono::*;

pub fn init(r: &str) -> Regex {
    let re = Regex::new(r).unwrap();
    re
}

pub fn monthIndex(line: &str) -> u32 {
    let mut parsed: Parsed = Parsed::new();
    //println!("{}", line);
    let items = vec![Item::Fixed(Fixed::ShortMonthName)];
    let items_iter = items.into_iter();
    format::parse(& mut parsed, line, items_iter);
    parsed.month.unwrap()
}

pub fn parse(re: &Regex, line: &str) -> Option<DateTime<UTC>> {
    let caps_o = re.captures(line);
    let caps = match caps_o {
        None => return None,
        Some(c) => c
    };
    let year = match caps.name("year") {
        None => return None,
        Some(y) => y.parse::<i32>().expect("could not parse year")
    };
    let mon = match caps.name("monthname")
        .map(|mn| monthIndex(mn))
        .or(caps.name("month").map(|t| t.parse::<u32>().expect("could not parse month")))
         {
        None => return None,
        Some(m) => m
    };
    let day = match caps.name("day") {
        None => return None,
        Some(d) => d.parse::<u32>().expect("could not parse day")
    };
    let hh = match caps.name("hour") {
        None => return None,
        Some(hh) => hh.parse::<u32>().expect("could not parse hour")
    };
    let mm = match caps.name("minute") {
        None => return None,
        Some(mm) => mm.parse::<u32>().expect("could not parse minute")
    };
    let ss = match caps.name("second") {
        None => return None,
        Some(ss) => ss.parse::<u32>().expect("could not parse second")
    };
    let dt = UTC.ymd(year, mon, day).and_hms(hh, mm, ss);
    Some(dt)
}

#[test]
fn test_sample_regexp() {
    let re = init(r"^\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})\]");
    let ti = parse(&re, "[2015-12-28 20:37:25]");
    assert_eq!(Some(2015), ti.map(|t| t.year()));
}

#[test]
fn test_nginx_regexp() {
    let re = init(r"^\[(?P<day>\d{2})/(?P<monthname>\p{L}*)/(?P<year>\d{4}):(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})");
    let ti = parse(&re, "[28/Dec/2015:20:37:25");
    assert_eq!(Some(2015), ti.map(|t| t.year()));
    assert_eq!(Some(12), ti.map(|t| t.month()));
}
