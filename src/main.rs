// API Documentation for The Movie Database DB:
// https://developers.themoviedb.org/3/getting-started/introduction
// https://developers.themoviedb.org/4/getting-started

// GET REQUEST EXAMPLES:
// curl --request GET   --url 'https://api.themoviedb.org/4/list/1'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'
// curl --request GET   --url 'https://api.themoviedb.org/4/search/tv?query=dragon'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'
// curl --request GET   --url 'https://api.themoviedb.org/3/tv/61709'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'

use reqwest::Client;
use fix_show_name::functions::{search_tv_show, get_tv_show_seasons};
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let show_name = "dragon ball";
    let show_id = 61709;

    let client = Client::new();
    
    // Search TV Shows
    let shows = search_tv_show(client, show_name).await?;

    // TODO: Should display a list of results of show + showid and the choice of picking the right option
    for show in shows {
        println!("{}", show)
    }

    
    // dbg!(shows);
    
    // let client = Client::new();
    // // Get season & episode information about TV Show
    // let show_details = get_tv_show_seasons(client, show_id).await?;


    Ok(())

}


