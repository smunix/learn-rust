use chrono::NaiveDate;
use csv::Reader;
use csv_serde::naive_date::from_deserializer;
use serde::Deserialize;

/// name,location,Date,Rating,Review,Image_Links
#[derive(Debug, Deserialize)]
struct Review {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "location")]
    location: String,
    #[serde(rename = "Date", deserialize_with = "from_deserializer")]
    date: NaiveDate,
    #[serde(rename = "Rating")]
    rating: String,
    #[serde(rename = "Review")]
    review: String,
    #[serde(rename = "Image_Links")]
    image_links: String,
}

fn main() {
    let mut rdr = Reader::from_path("data/starbucks/reviews_data.csv").unwrap();
    let reviews: Vec<Review> = rdr.deserialize().map(|r| r.unwrap()).collect();
    reviews.iter().take(10).for_each(|r| println!("{:?}", r))
}
