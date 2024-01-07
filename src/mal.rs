use core::fmt;

use serde_json::Value;

#[derive(Debug)]
pub struct AnimeList {
    id: i64,
    title: String,
    status: String,
}

impl AnimeList {
    pub fn new(value: &Value) -> Self {
        // parse node
        let (id, title) = Self::parse_node(value);
        let status = Self::parse_status(value);
        Self {
            id: id.unwrap(),
            title: title.unwrap().to_string(),
            status: status.unwrap().to_string(),
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

    fn parse_status(value: &Value) -> Option<&str> {
        let status = value.get("list_status");
        match status {
            Some(anime_status) => anime_status.get("status").unwrap().as_str(),
            _ => None,
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
        write!(f, "{}\t{}", self.status, self.title)
    }
}
