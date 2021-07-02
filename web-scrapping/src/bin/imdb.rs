use scraper::{Html, Selector};
use std::fmt::Write;

fn main() {
    println!("{:?}", get_info("tt5031232"));
}

//https://users.rust-lang.org/t/check-if-a-string-in-a-list-exist/29316
fn high_contain<'a>(mut strings: impl Iterator<Item = &'a str>, key: &'a str) -> bool {
    strings.any(|item| key.contains(item))
}

fn get_info(imdb_id: &str) -> (Option<f64>, Option<bool>) {
    let mut url = "https://www.imdb.com/title/".to_string();
    write!(url, "{}", imdb_id).unwrap();
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().unwrap();

    //For debugging purpose only
    //let mut debug_file = "/tmp/debug".to_string();
    //write!(debug_file, "{}", imdb_id).unwrap();
    //std::fs::write(debug_file, body.clone()).unwrap();

    let fragment = Html::parse_document(&body);
    get_info_with_fallback(fragment, false)
}

fn get_info_with_fallback(fragment: Html, fallback: bool) -> (Option<f64>, Option<bool>) {
    let not_theatrical = [
        "TV Movie",
        "TV Short",
        "Video",
        "Episode aired",
        "TV Series",
        "TV Special",
    ];

    let rating_selector = match fallback {
        false => Selector::parse(".TitleBlock__HideableRatingBar-sc-1nlhx7j-4 > div:nth-child(1) > div:nth-child(1) > a:nth-child(2) > div:nth-child(1) > div:nth-child(1) > div:nth-child(2) > div:nth-child(1) > span:nth-child(1)").unwrap(),
        true => Selector::parse(".ratingValue > strong:nth-child(1) > span:nth-child(1)").unwrap(),
    };

    let rating = fragment.select(&rating_selector).next();
    let rating = rating.map(|r| r.text().next().unwrap());
    let rating = rating.map(|r| r.parse::<f64>().unwrap());

    let theatrical_selector = match fallback {
        false => {
            Selector::parse(".TitleBlockMetaData__MetaDataList-sc-12ein40-0 > li:nth-child(1)")
                .unwrap()
        }
        true => Selector::parse(".subtext").unwrap(),
    };

    let raw_threatrical = fragment.select(&theatrical_selector).next();
    let theatrical = raw_threatrical.map(|r| {
        r.text()
            .into_iter()
            .any(|v| high_contain(std::array::IntoIter::new(not_theatrical), v))
    });

    if rating.is_none() && theatrical.is_none() && !fallback {
        get_info_with_fallback(fragment, true)
    } else {
        (rating, theatrical)
    }
}

#[test]
fn get_correct_ratings_and_detect_theatrical_film() {
    assert!(
        get_info("tt1390411") == (Some(6.9), Some(false)),
        "tt1390411"
    );
    assert!(
        get_info("tt0304584") == (Some(4.4), Some(true)),
        "tt0304584"
    );
    assert!(
        get_info("tt0827521") == (Some(5.5), Some(true)),
        "tt0827521"
    );
    assert!(get_info("tt0001539") == (None, Some(false)), "tt0001539");
    assert!(
        get_info("tt5031232") == (Some(8.8), Some(true)),
        "tt5031232"
    );
    assert!(
        get_info("tt4049416") == (Some(5.2), Some(true)),
        "tt4049416"
    );
}
