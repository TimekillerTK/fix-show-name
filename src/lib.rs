pub mod functions {
    use serde::{Deserialize, Serialize};
    use reqwest::{
        header::{HeaderMap, HeaderValue, AUTHORIZATION},
        Client
    };
    use std::env;

    #[derive(Debug, Serialize, Deserialize)]
    struct Response {
        results: Vec<TVShow>
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TVShow {
        pub id: i32,
        pub name: String,
        pub overview: String
    }

    pub async fn search_tv_show(client: Client, show_name: &str) -> Result<Vec<TVShow>, Box<dyn std::error::Error>> {

        let api_key = env::var("API_KEY").unwrap();
        let mut headers = HeaderMap::new();

        // Set the authorization header with the bearer token
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Authorization: Bearer {}", api_key)).unwrap());

        // Make the GET request with the headers
        // TODO: Needs to handle getting extra pages
        let resp = client
            .get("https://api.themoviedb.org/4/search/tv")
            .headers(headers)
            .query(&[("query", format!("{show_name}"))])
            .send()
            .await?
            .text()
            .await?;

        let response: Response = serde_json::from_str(&resp)?;
        
        Ok(response.results)
    }

    pub async fn get_tv_show_info(client: Client, show_id: i32) -> Result<(), Box<dyn std::error::Error>> {

        let api_key = env::var("API_KEY").unwrap();
        let mut headers = HeaderMap::new();

        // Set the authorization header with the bearer token
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Authorization: Bearer {}", api_key)).unwrap());

        // Make the GET request with the headers
        // TODO: Needs to handle getting extra pages
        let resp = client
            .get(format!("https://api.themoviedb.org/4/tv/{}", show_id))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        // let response: Response = serde_json::from_str(&resp)?;
        dbg!(resp);
        Ok(())
    }
}