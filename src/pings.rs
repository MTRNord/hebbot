use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
struct Response {
    disclaimer: String,
    pings: HashMap<String, Ping>,
    mean: f32,
    pongservers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Ping {
    pongs: Option<HashMap<String, Pong>>,
    pings: Vec<String>,
    mean: f32,
    median: f32,
    gmean: f32,
}

#[derive(Serialize, Deserialize)]
struct Pong {
    diffs: Option<HashMap<String, f32>>,
    mean: f32,
    median: f32,
    gmean: f32,
}

struct Score {
    server: String,
    median: f32,
}

pub struct Pings {}

impl Pings {
    fn sort_pings(ping_data: Response) -> Vec<Score> {
        let mut sorted_pings: Vec<Score> = ping_data
            .pings
            .iter()
            .map(|(k, v)| Score {
                server: k.to_string(),
                median: v.median,
            })
            .collect();
        sorted_pings.sort_by(|a, b| a.median.total_cmp(&b.median));
        sorted_pings
    }

    fn render_table(sorted_pings: Vec<Score>) -> String {
        let mut table = String::new();
        for (i, score) in sorted_pings.iter().take(10).enumerate() {
            table = format!("{table}|{}|{}|{}|\n", i + 1, score.server, score.median);
        }
        table
    }

    pub fn get(room_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let ping_data: Response = reqwest::blocking::get(format!(
            "https://maubot.xyz/_matrix/maubot/plugin/pingstat/{room_id}/stats.json"
        ))?
        .json()?;

        let sorted_pings = Pings::sort_pings(ping_data);
        Ok(Pings::render_table(sorted_pings))
    }
}

// Test cases specific to pings
#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Response {
        Response {
            disclaimer: String::new(),
            pings: HashMap::from([
                (
                    "server1".to_string(),
                    Ping {
                        pongs: None,
                        pings: vec![],
                        mean: 0f32,
                        median: 10f32,
                        gmean: 0f32,
                    },
                ),
                (
                    "server2".to_string(),
                    Ping {
                        pongs: None,
                        pings: vec![],
                        mean: 0f32,
                        median: 5f32,
                        gmean: 0f32,
                    },
                ),
            ]),
            mean: 0f32,
            pongservers: Vec::new(),
        }
    }

    #[test]
    fn ensure_sorted() {
        let ping_data = test_data();

        let sorted_pings = Pings::sort_pings(ping_data);
        assert_eq!(sorted_pings.len(), 2);
        assert!(sorted_pings[0].median <= sorted_pings[1].median);
    }

    #[test]
    fn ensure_table_format() {
        let ping_data = test_data();

        let sorted_pings = Pings::sort_pings(ping_data);
        let table = Pings::render_table(sorted_pings);
        let expected_table = "|1|server2|5|\n|2|server1|10|\n";
        assert_eq!(table, expected_table);
    }

    #[test]
    fn ensure_fetch_works() {
        let result = Pings::get("!ping12Z19lU3TzHS4slLsUNPx-I7MZKyneYRUlO7voU");
        assert!(result.is_ok());
        let table = result.unwrap();
        assert!(!table.is_empty());
    }
}
