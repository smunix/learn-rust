use serde::Deserialize;
use url::Url;

trait IsImageLink<T: AsRef<str>> {
    fn is_image_link(self) -> Option<Url>;
}

impl<T: AsRef<str>> IsImageLink<T> for T {
    fn is_image_link(self) -> Option<Url> {
        match self.as_ref() {
            "No Images" => None,
            x => Url::parse(x).ok(),
        }
    }
}

pub fn deserialize_from<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<Vec<Url>>, D::Error> {
    Ok(String::deserialize(d)
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(", ")
        .map(|lnk| {
            lnk.trim_start_matches('\'')
                .trim_end_matches('\'')
                .is_image_link()
        })
        .collect())
}
