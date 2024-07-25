use chrono::NaiveDate;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[repr(u32)]
enum Month {
    Jan = 1,
    Feb,
    March,
    April,
    May,
    June,
    July,
    Aug,
    Sept,
    Oct,
    Nov,
    Dec,
}

impl Into<u32> for Month {
    fn into(self) -> u32 {
        self as u32
    }
}

impl FromStr for Month {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Jan." => Ok(Month::Jan),
            "Feb." => Ok(Month::Feb),
            "March" => Ok(Month::March),
            "April" => Ok(Month::April),
            "May" => Ok(Month::May),
            "June" => Ok(Month::June),
            "July" => Ok(Month::July),
            "Aug." => Ok(Month::Aug),
            "Sept." => Ok(Month::Sept),
            "Oct." => Ok(Month::Oct),
            "Nov." => Ok(Month::Nov),
            "Dec." => Ok(Month::Dec),
            _ => Err(()),
        }
    }
}

trait IsNaiveDate {
    fn is_naive_date(&self) -> Result<NaiveDate, &'static str>;
}

impl IsNaiveDate for String {
    fn is_naive_date(&self) -> Result<NaiveDate, &'static str> {
        let ref mut it = self.trim_start_matches("Reviewed ").split_whitespace();
        let month = it
            .next()
            .expect("no month")
            .parse::<Month>()
            .expect("invalid month")
            .into();
        let day = it
            .next()
            .expect("no day")
            .trim_end_matches(',')
            .parse()
            .expect("invalid day");
        let year = it.next().expect("no year").parse().expect("invalid year");
        NaiveDate::from_ymd_opt(year, month, day).ok_or("failed to convert NaiveDate")
    }
}

pub fn from_deserializer<'de, D>(d: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(d)
        .expect("failed to read naive date")
        .is_naive_date()
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_convert() {
        const DATE_FORMAT: &str = "%b %d, %Y";
        assert_eq!(
            "Reviewed Sept. 13, 2023"
                .to_string()
                .is_naive_date()
                .unwrap(),
            NaiveDate::parse_from_str("Sep 13, 2023", DATE_FORMAT).expect("Sep 13, 2023")
        );
        assert_eq!(
            "Reviewed July 16, 2023"
                .to_string()
                .is_naive_date()
                .unwrap(),
            NaiveDate::parse_from_str("Jul 16, 2023", DATE_FORMAT).expect("Jul 16, 2023")
        );
        assert_eq!(
            "Reviewed Feb. 28, 2023"
                .to_string()
                .is_naive_date()
                .expect("Feb. 28, 2023"),
            NaiveDate::parse_from_str("Feb 28, 2023", DATE_FORMAT).expect("Feb 28, 2023")
        );
    }
}
