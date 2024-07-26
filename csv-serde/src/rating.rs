use serde::Deserialize;
use std::str::FromStr;

trait IsRating {
    fn is_rating(&self) -> Result<Option<i32>, &'static str>;
}

impl IsRating for String {
    fn is_rating(&self) -> Result<Option<i32>, &'static str> {
        self.parse::<i32>().map_or(Ok(None), |v| Ok(Some(v)))
    }
}

pub fn deserialize_from<'de, D>(d: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(d)
        .expect("rating")
        .is_rating()
        .map_err(serde::de::Error::custom)
}
