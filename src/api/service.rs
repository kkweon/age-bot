use chrono::{Date, NaiveDate, Utc};

pub fn parse_naive_date(text: &str) -> Option<NaiveDate> {
    let formats = ["%Y-%m-%d", "%Y/%m/%d", "%m/%d/%Y"];

    for format in formats.iter() {
        if let Ok(naive) = NaiveDate::parse_from_str(text, format) {
            return Some(naive);
        }
    }

    None
}

pub fn parse_utc_date(text: &str) -> Option<Date<Utc>> {
    parse_naive_date(text).map(|naive| Date::from_utc(naive, Utc))
}

pub fn days_since(date: Date<Utc>) -> i64 {
    let today = Utc::today();
    let diff = today - date;

    diff.num_days()
}

pub fn parse_and_get_age(text: &str) -> Option<i64> {
    let date = parse_utc_date(text)?;
    Some(days_since(date))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_naive_date() {
        let input = parse_naive_date("1988/08/13");
        let expected = NaiveDate::from_ymd(1988, 08, 13);

        assert_eq!(input.unwrap(), expected);

        let input = parse_naive_date("1988-08-13");
        let expected = NaiveDate::from_ymd(1988, 08, 13);

        assert_eq!(input.unwrap(), expected);
    }
}
