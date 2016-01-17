use regex::Regex;

pub fn matches(re: &Regex, line: &str) -> bool {
    re.is_match(line)
}

#[test]
fn test_sample_regexp() {
    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\]").unwrap();
    assert_eq!(true, matches(&re, "[2015-12-28 20:37:25] @30262 INFO: Processing by AgentController#tasks as"));
}