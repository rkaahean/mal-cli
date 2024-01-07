use std::error::Error;

use inquire::Select;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use crate::{auth::get_access_token, mal::AnimeList};

pub async fn show_list() -> Result<(), Box<dyn Error + Send>> {
    let access_token = get_access_token().await.unwrap();
    let token = access_token.trim_start_matches("\"").trim_end_matches("\"");
    let client = reqwest::Client::new();

    // post request
    let response = client
        .get("https://api.myanimelist.net/v2/users/@me/animelist?fields=list_status&limit=30")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let response_text = response.text().await.unwrap();
        let response_json: Value = serde_json::from_str(response_text.as_str()).unwrap();

        let anime_list = get_anime_list_from_json(&response_json);
        let _ = Select::new("Status\tCompleted at\tTitle", anime_list)
            .prompt()
            .and_then(|anime| Ok(open_url(&anime)));
    }
    Ok(())
}

pub fn get_anime_list_from_json(json: &Value) -> Vec<AnimeList> {
    let data: &Value = json.get("data").unwrap();


    data.as_array()
        .unwrap()
        .iter()
        .map(|anime| AnimeList::new(anime))
        .collect::<Vec<AnimeList>>()
}

pub fn open_url(anime: &AnimeList) {
    open::that(format!("https://myanimelist.net/anime/{}", anime.get_id())).unwrap();
}
