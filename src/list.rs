use std::error::Error;

use inquire::Select;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use async_recursion::async_recursion;

use crate::{
    auth::{get_access_token, reauthenticate},
    mal::{Anime, AnimeList},
};

#[async_recursion]
pub async fn show_list() -> Result<(), Box<dyn Error + Send>> {
    let access_token = get_access_token().await.unwrap();
    let client = reqwest::Client::new();

    // post request
    let response = client
        .get("https://api.myanimelist.net/v2/users/@me/animelist?fields=list_status&limit=30")
        .header(AUTHORIZATION, format!("Bearer {}", &access_token))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let response_text = response.text().await.unwrap();
        let response_json: Value = serde_json::from_str(response_text.as_str()).unwrap();

        let anime_list = parse_anime_list_from_json(&response_json);
        let anime = Select::new("Status\tCompleted at\tTitle", anime_list).prompt();

        match anime {
            Ok(anime) => show_anime_details(&anime).await,
            _ => (),
        }
    } else {
        // reuauthenticate and try again
        let _ = reauthenticate().await;
        show_list().await?;
    }
    Ok(())
}

pub fn parse_anime_list_from_json(json: &Value) -> Vec<AnimeList> {
    let data: &Value = json.get("data").unwrap();

    data.as_array()
        .unwrap()
        .iter()
        .map(|anime| AnimeList::new(anime))
        .collect::<Vec<AnimeList>>()
}

pub async fn show_anime_details(anime: &AnimeList) {
    /*
       Get details about the anime
    */
    let access_token = get_access_token().await.unwrap();
    let client = reqwest::Client::new();
    let response = client
            .get(
                format!(
                    "https://api.myanimelist.net/v2/anime/{}?fields=id,title,main_picture,alternative_titles,start_date,end_date,synopsis,mean,rank",
                    anime.get_id()
                )
            )
            .header(AUTHORIZATION, format!("Bearer {}", &access_token))
            .send()
            .await
            .unwrap();

    if response.status().is_success() {
        let response_text = response.text().await.unwrap();
        let anime: Anime = serde_json::from_str(&response_text).unwrap();
        println!("{}", anime);
    }
}

pub fn open_url(anime: &AnimeList) {
    open::that(format!("https://myanimelist.net/anime/{}", anime.get_id())).unwrap();
}
