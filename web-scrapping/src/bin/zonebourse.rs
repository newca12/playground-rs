use scraper::{Html, Selector};

fn main() {
    let stocks = [
        "ATOS-SE-4612",
        "BIC-4617",
        "DBV-TECHNOLOGIES-10189744",
        "EDF-4998",
        "GENFIT-16311755",
        "NICOX-25281955",
        "TECHNICOLOR-6411898",
        "WORLDLINE-16783982",
    ];
    for stock in std::array::IntoIter::new(stocks) {
        println!("{} : {:?}", stock, get_info(stock));
    }
}

fn get_info(zonebourse_id: &str) -> Vec<String> {
    let mut result = Vec::new();
    let resp = reqwest::blocking::get(format!(
        "https://www.zonebourse.com/cours/action/{}/consensus/",
        zonebourse_id
    ))
    .unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();
    let fragment = Html::parse_document(&body);
    let max_selector = Selector::parse(
        "tr.RC_tr0:nth-child(5) > td:nth-child(2) > b:nth-child(1) > font:nth-child(1)",
    )
    .unwrap();
    let moy_selector = Selector::parse(
        "tr.RC_tr1:nth-child(6) > td:nth-child(2) > b:nth-child(1) > font:nth-child(1)",
    )
    .unwrap();
    let min_selector = Selector::parse(
        ".Bord > tbody:nth-child(1) > tr:nth-child(7) > td:nth-child(2) > b:nth-child(1) > font:nth-child(1)",
    )
    .unwrap();

    let consensus_max = fragment.select(&max_selector).next();
    let consensus_max = consensus_max.unwrap().text().next();
    result.push(consensus_max.unwrap().to_string());
    let consensus_moy = fragment.select(&moy_selector).next();
    let consensus_moy = consensus_moy.unwrap().text().next();
    result.push(consensus_moy.unwrap().to_string());
    let consensus_min = fragment.select(&min_selector).next();
    let consensus_min = consensus_min.unwrap().text().next();
    result.push(consensus_min.unwrap().to_string());

    result
}
