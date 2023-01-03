// API Documentation for The Movie Database DB:
// https://developers.themoviedb.org/3/getting-started/introduction
// https://developers.themoviedb.org/4/getting-started

// GET REQUEST EXAMPLES:
// curl --request GET   --url 'https://api.themoviedb.org/4/list/1'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'
// curl --request GET   --url 'https://api.themoviedb.org/4/search/tv?query=dragon'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'
// curl --request GET   --url 'https://api.themoviedb.org/3/tv/61709'   --header "Authorization: Bearer ${API_KEY}"   --header 'Content-Type: application/json;charset=utf-8'

use fix_show_name::functions::{search_tv_show, get_tv_show_seasons, list_media_files};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let show_name = "dragon ball";
    let path = "testpath";
    
    // Search for TV Show
    let tv_shows = search_tv_show(show_name).await?;
    let selected_tv_show = 61459;

    // Display TV Show seasons
    let seasons = get_tv_show_seasons(selected_tv_show).await?;
    let selected_season = 62946;

    
    // List media files in directory
    let media_files = list_media_files(path);
    // dbg!(media_files); 

    // if 
    for (i, file) in media_files.file_names.iter().enumerate() {
        

        let new_file_name = format!("{}{}", file, "xxxx");
        
        let old_path = media_files.path.join(&file);
        let new_path = media_files.path.join(&new_file_name);
        println!("{} --> {}", file, new_file_name);
        // println!("{:?} --> {:?}", old_path, new_path);
        // fs::rename(old_path, new_path).unwrap();
    }

    // Target name: 
    // SHOWNAME - S1 - 001.mkv

    // TODO: Handle cases:
    // show has >   9 episodes =   0X
    // show has >  99 episodes =  00X
    // show has > 999 episodes = 000X

    Ok(())

}



