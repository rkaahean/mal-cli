use core::fmt;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
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
        // 9 because 2023-07-06 has 10 characters
        write!(
            f,
            "{}\t{:>9}\t{}",
            self.status, self.finish_date, self.title
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Anime {
    id: i64,
    title: String,
    start_date: String,
    end_date: String,
    synopsis: String,
    mean: f64,
    rank: i64,
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 9 because 2023-07-06 has 10 characters
        write!(
            f,
            "Title\t{}\nStart\t{}\nEnd\t{}\nScore\t{}\n",
            self.title, self.start_date, self.end_date, self.mean
        )
    }
}
