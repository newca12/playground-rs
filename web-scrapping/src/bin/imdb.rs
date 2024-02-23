use reqwest::{Client, Url};
use scraper::{Html, Selector};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let imdb_id = vec![
        "tt1390411",
        "tt0304584",
        "tt0827521",
        "tt0001539",
        "tt5031232",
        "tt4049416",
        "tt17061910",
    ];

    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0",
        )
        .build()
        .unwrap();

    let mut join_set = JoinSet::new();
    for id in imdb_id {
        let client = client.clone();
        join_set.spawn(async move { println!("{} {:?}", id, get_info(client, id).await) });
    }
    while let Some(_res) = join_set.join_next().await {}
    Ok(())
}

//https://users.rust-lang.org/t/check-if-a-string-in-a-list-exist/29316
fn high_contain<'a>(mut strings: impl Iterator<Item = &'a str>, key: &'a str) -> bool {
    strings.any(|item| key.contains(item))
}

async fn get_info(client: Client, id: &str) -> (Option<f64>, Option<bool>) {
    let url = format!("https://www.imdb.com/title/{}", id);
    let resp = client
        .get(Url::parse(url.as_str()).unwrap())
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();

    //For debugging purpose only
    //let mut debug_file = "/tmp/debug".to_string();
    //write!(debug_file, "{}", url).unwrap();
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

    //
    let rating_selector = match fallback {
        false => Selector::parse("div.sc-3a4309f8-0:nth-child(2) > div:nth-child(1) > div:nth-child(1) > a:nth-child(2) > span:nth-child(1) > div:nth-child(1) > div:nth-child(2) > div:nth-child(1) > span:nth-child(1)").unwrap(),
        true => Selector::parse("div.sc-c6e5278a-0:nth-child(2) > div:nth-child(1) > div:nth-child(1) > a:nth-child(2) > span:nth-child(1) > div:nth-child(1) > div:nth-child(2) > div:nth-child(1) > span:nth-child(1)").unwrap(),
    };

    let rating = fragment.select(&rating_selector).next();
    let rating = rating.map(|r| r.text().next().unwrap());
    let rating = rating.map(|r| r.parse::<f64>().unwrap());

    let theatrical_selector = match fallback {
        false => Selector::parse("ul.ipc-inline-list--show-dividers:nth-child(2)").unwrap(),
        true => Selector::parse("ul.ipc-inline-list:nth-child(3) > li:nth-child(1)").unwrap(),
    };

    let raw_threatrical = fragment.select(&theatrical_selector).next();
    let theatrical = raw_threatrical.map(|r| {
        r.text()
            .any(|v| high_contain(IntoIterator::into_iter(not_theatrical), v))
    });

    if (rating.is_none() || theatrical.is_none()) && !fallback {
        let (r, t) = get_info_with_fallback(fragment, true);
        let sr = if r.is_none() { rating } else { r };
        let st = if t.is_none() { theatrical } else { t };
        (sr, st)
    } else {
        (rating, theatrical)
    }
}
#[tokio::test]
async fn get_correct_ratings_and_detect_theatrical_film() {
    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0",
        )
        .build()
        .unwrap();
    assert!(
        get_info(client.clone(), "tt1390411").await == (Some(6.9), Some(false)),
        "tt1390411"
    );
    assert!(
        get_info(client.clone(), "tt0304584").await == (Some(4.3), Some(true)),
        "tt0304584"
    );
    assert!(
        get_info(client.clone(), "tt0827521").await == (Some(5.5), Some(true)),
        "tt0827521"
    );
    assert!(
        get_info(client.clone(), "tt0001539").await == (None, Some(false)),
        "tt0001539"
    );
    assert!(
        get_info(client.clone(), "tt5031232").await == (Some(8.6), Some(true)),
        "tt5031232"
    );
    assert!(
        get_info(client.clone(), "tt4049416").await == (Some(5.2), Some(true)),
        "tt4049416"
    );
    assert!(
        get_info(client.clone(), "tt17061910").await == (Some(7.1), Some(true)),
        "redirect tt17061910"
    );
}
