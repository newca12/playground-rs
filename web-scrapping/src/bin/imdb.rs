use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

const IMDB_GRAPHQL_URL: &str = "https://caching.graphql.imdb.com/";

#[derive(Serialize)]
struct GraphQLQuery {
    query: String,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(Deserialize, Debug)]
struct GraphQLData {
    title: Option<TitleData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TitleData {
    ratings_summary: Option<RatingsSummary>,
    title_type: Option<TitleType>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RatingsSummary {
    aggregate_rating: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct TitleType {
    text: Option<String>,
}

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

    let client = build_client();

    let mut join_set = JoinSet::new();
    for id in imdb_id {
        let client = client.clone();
        join_set.spawn(async move { println!("{} {:?}", id, get_info(client, id).await) });
    }
    while let Some(_res) = join_set.join_next().await {}
    Ok(())
}

fn build_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0")
        .default_headers(headers)
        .build()
        .unwrap()
}

/// Checks if the title type indicates a non-theatrical release
fn is_non_theatrical(title_type: &str) -> bool {
    let not_theatrical = [
        "TV Movie",
        "TV Short",
        "Video",
        "TV Episode",
        "TV Series",
        "TV Special",
        "TV Mini Series",
    ];
    not_theatrical.iter().any(|&t| title_type.contains(t))
}

async fn get_info(client: Client, id: &str) -> (Option<f64>, Option<bool>) {
    let query = format!(
        r#"query {{ title(id: "{}") {{ ratingsSummary {{ aggregateRating }} titleType {{ text }} }} }}"#,
        id
    );

    let graphql_query = GraphQLQuery { query };

    let resp = client
        .post(IMDB_GRAPHQL_URL)
        .json(&graphql_query)
        .send()
        .await
        .unwrap();

    if !resp.status().is_success() {
        return (None, None);
    }

    let body: GraphQLResponse = match resp.json().await {
        Ok(body) => body,
        Err(_) => return (None, None),
    };

    let title = match body.data.title {
        Some(t) => t,
        None => return (None, None),
    };

    let rating = title.ratings_summary.and_then(|rs| rs.aggregate_rating);

    let theatrical = title
        .title_type
        .and_then(|tt| tt.text.map(|text| is_non_theatrical(&text)));

    (rating, theatrical)
}

#[tokio::test]
async fn get_correct_ratings_and_detect_theatrical_film() {
    let client = build_client();
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
        get_info(client.clone(), "tt4049416").await == (Some(5.3), Some(true)),
        "tt4049416"
    );
    assert!(
        get_info(client.clone(), "tt17061910").await == (Some(7.1), Some(true)),
        "redirect tt17061910"
    );
}
