use scraper::{Html, Selector};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stocks = [
        "ATOS-SE-4612",
        "BIC-4617",
        "DBV-TECHNOLOGIES-10189744",
        "GENFIT-16311755",
        "NICOX-SA-25281955",
        "VANTIVA-6411898",
        "WORLDLINE-16783982",
    ];

    let mut join_set = JoinSet::new();
    for stock in IntoIterator::into_iter(stocks) {
        join_set.spawn(async move { display(stock).await });
    }
    while let Some(_res) = join_set.join_next().await {}
    Ok(())
}

async fn get_info(zonebourse_id: &str) -> Vec<String> {
    let mut result = Vec::new();
    let resp = reqwest::get(format!(
        "https://www.zonebourse.com/cours/action/{}/consensus/",
        zonebourse_id
    ))
    .await
    .unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().await.unwrap();
    let fragment = Html::parse_document(&body);
    let selectors = [
        "div.grid:nth-child(10) > div:nth-child(2) > span:nth-child(1)",
        "div.grid:nth-child(6) > div:nth-child(2) > span:nth-child(1)",
        "div.grid:nth-child(8) > div:nth-child(2) > span:nth-child(1)",
    ];

    for selector in IntoIterator::into_iter(selectors) {
        let selector = Selector::parse(selector).unwrap();
        let consensus = fragment.select(&selector).next();
        let consensus = consensus.unwrap().text().next();
        result.push(consensus.unwrap().to_string());
    }

    result
}

async fn display(stock: &str) {
    let infos = get_info(stock).await;
    let infos: Vec<String> = infos
        .iter()
        .map(|info| info.replace('\u{a0}', ""))
        .collect();
    println!("{} : {:?}", stock, infos);
}
