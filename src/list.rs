use std::error::Error;

use inquire::formatter::OptionFormatter;
use inquire::Select;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use crate::{
    auth::{get_access_token, reauthenticate},
    mal::{Anime, AnimeList},
};

pub struct ListArgs {
    num: Option<i32>,
}

impl ListArgs {
    pub fn new(num: Option<i32>) -> Self {
        Self { num }
    }
}

pub async fn show_list(args: ListArgs) -> Result<(), Box<dyn Error + Send>> {
    let access_token = get_access_token().await.unwrap();
    let client = reqwest::Client::new();

    let num_anime = match args.num {
        Some(num) => num,
        _ => 10,
    };
    // post request
    let response = client
        .get(format!("https://api.myanimelist.net/v2/users/@me/animelist?fields=list_status&limit={num_anime}"))
        .header(AUTHORIZATION, format!("Bearer {}", &access_token))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let response_text = response.text().await.unwrap();
        let response_json: Value = serde_json::from_str(response_text.as_str()).unwrap();

        let formatter: OptionFormatter<AnimeList> =
            &|i| format!("\nSelected - {}", (*i.value).clone().get_title());
        loop {
            let anime_list = parse_anime_list_from_json(&response_json);
            let anime = Select::new("Status\tCompleted\tTitle", anime_list)
                .with_page_size(10)
                .with_formatter(formatter)
                .prompt();

            match anime {
                Ok(anime) => show_anime_details(anime.get_id()).await,
                Err(_) => {
                    break;
                }
            }
        }
    } else {
        // reuauthenticate and try again
        let _ = reauthenticate().await;
        println!("Reauthenticated token. Please try again...")
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

pub async fn show_anime_details(id: i64) {
    /*
       Get details about the anime
    */
    let access_token = get_access_token().await.unwrap();
    let client = reqwest::Client::new();
    let response = client
            .get(
                format!(
                    "https://api.myanimelist.net/v2/anime/{}?fields=id,title,start_date,end_date,synopsis,mean,rank",
                    id
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

    let options: Result<String, inquire::InquireError> = Select::new(
        "Options",
        vec!["Go back".to_string(), "Open MAL page".to_string()],
    )
    .prompt();

    match options {
        Ok(choice) => {
            if choice == "Open MAL page" {
                open_url(id);
            }
        }
        _ => (),
    }
}

pub fn open_url(id: i64) {
    open::that(format!("https://myanimelist.net/anime/{}", id)).unwrap();
}
