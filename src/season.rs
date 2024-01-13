use std::error::Error;

use chrono::{Datelike, Utc};
use inquire::{formatter::OptionFormatter, Select};
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use crate::{auth::get_access_token, list::show_anime_details, mal::SeasonalAnime};

pub struct SeasonArgs {
    season: Option<String>,
    year: Option<i32>,
}

impl SeasonArgs {
    pub fn new(season: Option<String>, year: Option<i32>) -> Self {
        Self { season, year }
    }
}

pub async fn show_season(args: SeasonArgs) -> Result<(), Box<dyn Error + Send>> {
    /*
       Shows the anime(s) for the season
    */

    let access_token = get_access_token().await.unwrap();
    let client = reqwest::Client::new();

    let season = match args.season {
        Some(season) => season,
        _ => "fall".to_string(),
    };
    let year = match args.year {
        Some(year) => year,
        _ => Utc::now().year(),
    };

    let response = client
        .get(format!(
            "https://api.myanimelist.net/v2/anime/season/{year}/{season}?limit=10&sort=anime_score",
        ))
        .header(AUTHORIZATION, format!("Bearer {}", &access_token))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let response_text = response.text().await.unwrap();
        let response_json: Value = serde_json::from_str(response_text.as_str()).unwrap();

        let formatter: OptionFormatter<SeasonalAnime> =
            &|i| format!("\nSelected - {}", (*i.value).clone().get_title());
        loop {
            let anime_list = parse_seasonal_anime_from_json(&response_json);
            let anime = Select::new("ID\tTitle", anime_list)
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
    }
    Ok(())
}

pub fn parse_seasonal_anime_from_json(json: &Value) -> Vec<SeasonalAnime> {
    let data: &Value = json.get("data").unwrap();

    data.as_array()
        .unwrap()
        .iter()
        .map(|anime| serde_json::from_value(anime.get("node").unwrap().clone()).unwrap())
        .collect::<Vec<SeasonalAnime>>()
}
