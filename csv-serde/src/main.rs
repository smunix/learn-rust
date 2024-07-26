use chrono::NaiveDate;
use csv::Reader;
use serde::Deserialize;
use url::Url;

/// name,location,Date,Rating,Review,Image_Links
#[derive(Debug, Deserialize)]
struct Review {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "location")]
    location: String,
    #[serde(
        rename = "Date",
        deserialize_with = "csv_serde::naive_date::deserialize_from"
    )]
    date: NaiveDate,
    #[serde(
        rename = "Rating",
        deserialize_with = "csv_serde::rating::deserialize_from"
    )]
    rating: Option<i32>,
    #[serde(rename = "Review")]
    review: String,
    #[serde(
        rename = "Image_Links",
        deserialize_with = "csv_serde::image_links::deserialize_from"
    )]
    image_links: Option<Vec<Url>>,
}

fn main() {
    let mut rdr = Reader::from_path("data/starbucks/reviews_data.csv").unwrap();
    let mut reviews = rdr
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<Review>>();
    reviews.sort_by(|b, a| (a.rating, a.date).cmp(&(b.rating, b.date)));
    let reviews = reviews
        .into_iter()
        .filter(|r| r.rating.is_some())
        .enumerate();
    reviews.for_each(|r| println!("{:?}", r));
}
