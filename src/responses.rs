// {
//     "lat": "39.91778",
//     "lon": "-75.241295",
//     "trainno": "219",
//     "service": "LOCAL",
//     "dest": "Wilmington",
//     "currentstop": "Penn Medicine Station",
//     "nextstop": "Darby",
//     "line": "Wilmington/Newark",
//     "consist": "401,330,331,384,385",
//     "heading": 227.33208020847601,
//     "late": 8,
//     "SOURCE": "Conshohocken",
//     "TRACK": "",
//     "TRACK_CHANGE": ""
//   }

use serde_derive::Deserialize;

use crate::deserialize::deserialize_csv_encoded_string;

pub type TrainResponse = Vec<Train>;

#[derive(Debug, Deserialize)]
pub struct Train {
    pub lat: String,
    pub lon: String,

    #[serde(rename = "trainno")]
    pub train_number: String,
    pub service: String,
    pub dest: String,

    #[serde(rename = "currentstop")]
    pub current_stop: String,

    #[serde(rename = "nextstop")]
    pub next_stop: String,

    pub line: String,

    #[serde(deserialize_with = "deserialize_csv_encoded_string")]
    pub consist: Vec<i32>,

    pub heading: Option<f64>,

    pub late: i32,

    #[serde(rename = "SOURCE")]
    pub source: String,

    #[serde(rename = "TRACK")]
    pub track: String,

    #[serde(rename = "TRACK_CHANGE")]
    pub track_change: String,
}
