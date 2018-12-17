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

pub fn get_message(days: i64) -> String {
    format!("You are {:.2} years old. Average life expectancy is 78.7 years. You have lived {:.2}% of your life", (days as f64) / 365f64, (days as f64)/(78.7f64 * 365f64) * 100f64)
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

        let input = parse_naive_date("8/13/1988");
        let expected = NaiveDate::from_ymd(1988, 08, 13);

        assert_eq!(input.unwrap(), expected);
    }

    #[test]
    fn test_get_message() {
        let input = 365_i64;
        assert_eq!(get_message(input), "You are 1.00 years old. Average life expectancy is 78.7 years. You have lived 1.27% of your life".to_owned())
    }
}
