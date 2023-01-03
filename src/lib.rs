pub mod functions {
    use serde::{Deserialize, Serialize};
    use reqwest::{
        header::{HeaderMap, HeaderValue, AUTHORIZATION},
        Client
    };
    use core::fmt;
    use std::{env, fmt::Display};

    #[derive(Debug, Serialize, Deserialize)]
    struct ResponseTVShow {
        results: Vec<TVShow>
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ResponseSeasons {
        seasons: Vec<Seasons>
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TVShow {
        pub id: i32,
        pub name: String,
        pub overview: String,
    }

    impl Display for TVShow {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "ID: {:7}, Name: {}",
            self.id, self.name)
        }

    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Seasons {
        pub id: i32,
        pub name: String,
        pub season_number: i32,
        pub episode_count: i32
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

        let response: ResponseTVShow = serde_json::from_str(&resp)?;
        
        Ok(response.results)
    }

    pub async fn get_tv_show_seasons(client: Client, show_id: i32) -> Result<Vec<Seasons>, Box<dyn std::error::Error>> {

        let api_key = env::var("API_KEY").unwrap();
        let mut headers = HeaderMap::new();

        // Set the authorization header with the bearer token
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Authorization: Bearer {}", api_key)).unwrap());

        // Make the GET request with the headers
        // TODO: Needs to handle getting extra pages
        let resp = client
            .get(format!("https://api.themoviedb.org/3/tv/{}", show_id))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let response: ResponseSeasons = serde_json::from_str(&resp)?;

        Ok(response.seasons)
    }
}