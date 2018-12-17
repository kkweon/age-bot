use chrono::NaiveDate;

pub fn parse_date(text: &str) -> NaiveDate{
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        let input = "1988/08/13";
        let expected = NaiveDate::from_ymd(1988, 08, 13);

        assert_eq!(input, expected);
    }
}
