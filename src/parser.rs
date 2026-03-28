use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    character::complete::u32,
    sequence::preceded,
};

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

fn parse_date_impl(input: &str) -> IResult<&str, Date> {
    const TAG: &str = "DATE PAYABLE: ";
    const SEPARATOR: &str = "/";
    let (input, _) = take_until(TAG).parse(input)?;
    let (input, (year, _, month, _, day)) =
        preceded(tag(TAG), (u32, tag(SEPARATOR), u32, tag(SEPARATOR), u32)).parse(input)?;

    Ok((input, Date::new(year, month, day)))
}

pub fn parse_date(input: &str) -> Option<Date> {
    match parse_date_impl(input) {
        Ok((_, date)) => Some(date),
        Err(_) => None,
    }
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
        assert_eq!(expected_result, parse_date(input));
    }
}
