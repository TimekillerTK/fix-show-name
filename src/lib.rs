pub mod functions {
    use serde::{Deserialize, Serialize};
    use reqwest::{
        header::{HeaderMap, HeaderValue, AUTHORIZATION},
        Client
    };
    use core::fmt;
    use std::{env, fmt::Display};
    use std::path::Path;
    use std::fs;
    use anyhow::anyhow;

    #[derive(Debug, Serialize, Deserialize)]
    struct ResponseTVShow {
        results: Vec<TVShow>,
        total_results: u32,
        total_pages: u32
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ResponseSeasons {
        seasons: Vec<Season>
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TVShow {
        pub id: i32,
        pub name: String,
        pub overview: String,
    }

    impl Display for TVShow {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "ID: {:6}, Name: {}",
            self.id, self.name)
        }

    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Season {
        pub id: i32,
        pub name: String,
        pub season_number: u32,
        pub episode_count: u32
    }

    impl Display for Season {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "ID: {:6}, Season: {} ({}), Eps: {}",
            self.id, self.name, self.season_number, self.episode_count)
        }

    }

    #[derive(Debug)]
    pub struct VideoFiles<'a> {
        pub path: &'a Path,
        pub file_names: Vec<String>
    }


    pub async fn search_tv_show(show_name: &str) -> Result<Vec<TVShow>, Box<dyn std::error::Error>> {
        let response_tv_show = query_moviedb_tvshow(show_name).await?;
        display_tvshow_response(&response_tv_show);
        Ok(response_tv_show.results)
    }

    async fn query_moviedb_tvshow(show_name: &str) -> Result<ResponseTVShow, Box<dyn std::error::Error>> {
        let client = Client::new();
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
        Ok(response)
    }

    fn display_tvshow_response(response: &ResponseTVShow)  {
        match response.total_results {
            0                   => println!("No results found!"),
            1                   => println!("{}", response.results[0]),
            2_u32..=u32::MAX    => {
                for show in &response.results {
                    println!("{}", show);
                }
            },
        }
    }


    pub async fn get_tv_show_seasons(show_id: i32) -> anyhow::Result<Vec<Season>> {

        let client = Client::new();
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

        match response.seasons.len() {
            0 => {
                Err(anyhow!("No results found!"))
            },
            1..=usize::MAX => {
                for season in &response.seasons {
                    println!("{}", season);
                }
                Ok(response.seasons)
            }
            _ => Err(anyhow!("Unknown error!"))
        }
        // Ok(response.seasons)
    }

    pub fn list_media_files(path: &str) -> VideoFiles {

        let valid_extensions = [".mkv", ".mp4", ".avi", ".mov"];
        let directory_path = Path::new(path);
        let mut video_files: VideoFiles = VideoFiles { path: directory_path, file_names: Vec::new()};
        
        let target_dir = fs::read_dir(directory_path)
            .unwrap();

        for file in target_dir {
            let file = file.unwrap();
            let path = file.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                for extension in valid_extensions {
                    if file_name.ends_with(extension) {
                        video_files.file_names.push(file_name.to_string());
                    }
                }
            }
        }
    
        video_files.file_names.sort_by(|x, y| x.cmp(y));
        video_files

    }

}