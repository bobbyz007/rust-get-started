use regex::Regex;

fn main() {
    regex_match();
    regex_group_name();
}

//  正则表达式匹配
pub fn regex_match() {
    // r: raw string
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let hay = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";

    let mut dates = vec![];
    for (_, [year, month, day]) in re.captures_iter(hay).map(|c| c.extract()) {
        dates.push((year, month, day));
    }
    assert_eq!(dates, vec![
        ("2010", "03", "14"),
        ("2014", "10", "14"),
    ]);
}

pub fn regex_group_name() {
    // ?x: 忽略空格并允许注释
    let re = Regex::new(r"(?x)
    (?P<year>\d{4})  # the year
    -
    (?P<month>\d{2}) # the month
    -
    (?P<day>\d{2})   # the day
    ").unwrap();

    let caps = re.captures("2010-03-14").unwrap();
    assert_eq!("2010", &caps["year"]);
    assert_eq!("03", &caps["month"]);
    assert_eq!("14", &caps["day"]);
}