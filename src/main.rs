// API Documentation for The Movie Database DB:
// https://developers.themoviedb.org/3/getting-started/introduction
// https://developers.themoviedb.org/4/getting-started

// GET REQUEST EXAMPLES:
// curl --request GET   --url 'https://api.themoviedb.org/4/list/1'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'
// curl --request GET   --url 'https://api.themoviedb.org/4/search/tv?query=dragon'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client
};
use serde::{Deserialize, Serialize};
use tokio;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    results: Vec<TVShow>
}

#[derive(Debug, Serialize, Deserialize)]
struct TVShow {
    id: i32,
    name: String,
    overview: String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let api_key = env::var("API_KEY").unwrap();
    let query_string = "dragon ball";

    let client = Client::new();
    let mut headers = HeaderMap::new();

    // Set the authorization header with the bearer token
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Authorization: Bearer {}", api_key)).unwrap());

    // Make the GET request with the headers
    // TODO: Needs to handle getting extra pages
    let resp = client
        .get("https://api.themoviedb.org/4/search/tv")
        .headers(headers)
        .query(&[("query", format!("{query_string}"))])
        .send()
        .await?
        .text()
        .await?;

    let response: Response = serde_json::from_str(&resp)?;
    
    for result in response.results {
        println!("{:?}", result)
    }

    Ok(())

}


