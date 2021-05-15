use reqwest;
use scraper::{Html, Selector};
use std::fmt::Write;

fn main() {
    println!("{:?}", get_info("tt5031232"));
}

fn high_contain<'a>(mut strings: impl Iterator<Item = &'a str>, key: &'a str) -> bool {
    strings.any(|item| key.contains(item))
}

fn get_info(imdb_id: &str) -> (Option<f64>, bool) {
    let not_theatrical = [
        "TV Movie",
        "TV Short",
        "Video",
        "Episode aired",
        "TV Series",
        "TV Special",
    ];

    let mut url = "https://www.imdb.com/title/".to_string();
    write!(url, "{}", imdb_id).unwrap();
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();
    let fragment = Html::parse_document(&body);

    let rating_selector =
        Selector::parse(".ratingValue > strong:nth-child(1) > span:nth-child(1)").unwrap();

    let rating = fragment.select(&rating_selector).next();
    let rating = if rating.is_some() {
        rating.unwrap().text().next()
    } else {
        None
    };
    let rating: Option<f64> = if rating.is_some() {
        Some(rating.unwrap().parse::<f64>().unwrap())
    } else {
        None
    };

    let theatrical_selector = Selector::parse(".subtext").unwrap();

    let raw_threatrical = fragment.select(&theatrical_selector).next();
    let theatrical = if raw_threatrical
        .unwrap()
        .text()
        .into_iter()
        .find(|v| high_contain(std::array::IntoIter::new(not_theatrical), v))
        .is_some()
    {
        true
    } else {
        false
    };

    (rating, theatrical)
}

#[test]
fn get_correct_ratings_and_detect_theatrical_film() {
    assert!(get_info("tt1390411") == (Some(6.9), false));
    assert!(get_info("tt0304584") == (Some(4.4), true));
    assert!(get_info("tt0827521") == (Some(5.6), true));
    assert!(get_info("tt0001539") == (None, false));
    assert!(get_info("tt5031232") == (Some(8.8), true));
    assert!(get_info("tt4049416") == (Some(5.3), true));
}