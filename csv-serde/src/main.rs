use std::{fs::File, io::BufWriter};

use chrono::NaiveDate;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use url::Url;

/// name,location,Date,Rating,Review,Image_Links
#[derive(Debug, Deserialize, Serialize)]
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

fn rd() -> Vec<(usize, Review)> {
    let mut rdr = Reader::from_path("data/starbucks/reviews_data.csv").unwrap();
    let mut reviews = rdr
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<Review>>();
    reviews.sort_by(|b, a| (a.rating, a.date).cmp(&(b.rating, b.date)));
    let reviews = reviews
        .into_iter()
        // .filter(|r| r.rating.is_some())
        .enumerate();
    reviews.collect()
}

fn wr(reviews: &Vec<(usize, Review)>) {
    let f = File::create("data/starbucks/reviews_data.json").unwrap();
    let w = BufWriter::new(f);
    serde_json::to_writer_pretty(w, reviews).unwrap()
}

fn main() {
    wr(&rd())
}
