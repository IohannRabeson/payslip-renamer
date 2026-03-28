#[derive(Debug, PartialEq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

pub fn parse_date(input: &str, pattern: &str) -> Option<Date> {
    let re = regex::Regex::new(pattern).unwrap();
    let caps = re.captures(input)?;

    Some(Date::new(
        caps[1].parse().ok()?,
        caps[2].parse().ok()?,
        caps[3].parse().ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{Date, parse_date};

    #[rstest]
    #[case("KJDSFLJGH DATE PAYABLE: 2026/03/27 KjhasKJflh", Some(Date::new(2026, 03, 27)))]
    #[case("KJDSFLJGH DATE PAYABLE KjhasKJflh", None)]
    #[case("KJDSFLJGH DATE PAYABLE 2026/03/ KjhasKJflh", None)]
    fn test_parse_date(#[case] input: &str, #[case] expected_result: Option<Date>) {
        const PATTERN: &str = r"DATE PAYABLE: (\d{4})/(\d{2})/(\d{2})";

        assert_eq!(expected_result, parse_date(input, PATTERN));
    }
}
