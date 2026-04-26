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
    pub fn get(room_id: &str) -> String {
        let ping_json = reqwest::blocking::get(format!(
            "https://maubot.xyz/_matrix/maubot/plugin/pingstat/{room_id}/stats.json"
        ))
        .expect("request should succeed")
        .text()
        .expect("response should have text");
        let ping_data: Response = serde_json::from_str(&ping_json).expect("should parse as JSON");
        let mut sorted_pings: Vec<Score> = Vec::new();
        for (k, v) in ping_data.pings.iter() {
            sorted_pings.push(Score {
                server: k.to_string(),
                median: v.median,
            });
        }
        sorted_pings.sort_by(|a, b| a.median.total_cmp(&b.median));
        let mut table = String::from("");
        for (i, score) in sorted_pings[0..10].iter().enumerate() {
            table = format!("{table}|{}|{}|{}|\n", i + 1, score.server, score.median);
        }
        return table
    }
}
