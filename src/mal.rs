use core::fmt;

use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct AnimeList {
    id: i64,
    title: String,
    status: String,
    finish_date: String,
}

impl AnimeList {
    pub fn new(value: &Value) -> Self {
        // parse node
        let (id, title) = Self::parse_node(value);
        let (status, finish_date) = Self::parse_status(value);
        Self {
            id: id.unwrap(),
            title: title.unwrap().to_string(),
            status: status.unwrap().to_string(),
            finish_date: finish_date.unwrap().to_string(),
        }
        // todo!()
    }

    fn parse_node(value: &Value) -> (Option<i64>, Option<&str>) {
        let node = value.get("node");
        match node {
            Some(anime) => (
                anime.get("id").unwrap().as_i64(),
                anime.get("title").unwrap().as_str(),
            ),
            _ => (None, None),
        }
    }

    fn parse_status(value: &Value) -> (Option<&str>, Option<&str>) {
        let status = value.get("list_status");
        match status {
            Some(anime_status) => (
                anime_status.get("status").unwrap().as_str(),
                if let Some(date) = anime_status.get("finish_date") {
                    date.as_str()
                } else {
                    Some("-")
                },
            ),
            _ => (None, None),
        }
    }

    pub fn get_title(self) -> String {
        self.title
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_status(self) -> String {
        self.status
    }
}

impl fmt::Display for AnimeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut finish_date = "-".to_string();
        if self.finish_date != "-" {
            finish_date = NaiveDate::parse_from_str(self.finish_date.as_str(), "%Y-%m-%d")
                .unwrap()
                .format("%b %d, %Y")
                .to_string()
        }

        // 9 because 2023-07-06 has 10 characters
        write!(f, "{}\t{:>9}\t{}", self.status, finish_date, self.title)
    }
}

#[derive(Debug, Deserialize)]
pub struct Anime {
    id: i64,
    title: String,
    start_date: String,
    end_date: Option<String>,
    synopsis: String,
    mean: f64,
    rank: i64,
}

impl Anime {
    pub fn get_id(self) -> i64 {
        self.id
    }

    pub fn get_title(self) -> String {
        self.title
    }

    pub fn get_synopsis(self) -> String {
        self.synopsis
    }

    pub fn get_rank(self) -> i64 {
        self.rank
    }
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut start_date = "-".to_string();
        if self.start_date != "-" {
            start_date = NaiveDate::parse_from_str(self.start_date.as_str(), "%Y-%m-%d")
                .unwrap()
                .format("%b %d, %Y")
                .to_string()
        }

        let end_date = match &self.end_date {
            Some(date) => NaiveDate::parse_from_str(&date.as_str(), "%Y-%m-%d")
                .unwrap()
                .format("%b %d, %Y")
                .to_string(),
            _ => "-".to_string(),
        };

        // 9 because 2023-07-06 has 10 characters
        write!(
            f,
            "Start\t{}\nEnd\t{}\nScore\t{}\n",
            start_date, end_date, self.mean
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SeasonalAnime {
    id: i64,
    title: String,
}

impl SeasonalAnime {
    pub fn get_id(self) -> i64 {
        self.id
    }

    pub fn get_title(self) -> String {
        self.title
    }
}

impl fmt::Display for SeasonalAnime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.id, self.title)
    }
}
