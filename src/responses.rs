use serde_derive::Deserialize;

use crate::{
    deserialize::{deserialize_csv_encoded_string, deserialize_string_enum},
    types::{RegionalRailStop, RegionalRailsLine},
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Error(ApiError),
    Response(T),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct ArrivalsResponse {
    #[serde(rename = "Northbound")]
    pub northbound: Option<Vec<Arrivals>>,

    #[serde(rename = "Southbound")]
    pub southbound: Option<Vec<Arrivals>>,
}

#[derive(Debug, Deserialize)]
pub struct Arrivals {
    pub direction: String,
    pub path: String,
    pub train_id: String,
    pub origin: String,
    pub destination: String,
    pub line: RegionalRailsLine,
    pub status: String,
    pub service_type: String,
    pub next_station: String,
    pub sched_time: String,
    pub depart_time: String,
    pub track: String,
    pub track_change: Option<String>,
    pub platform: String,
    pub platform_change: Option<String>,
}

pub type TrainApiResponse = ApiResponse<TrainResponse>;
pub type TrainResponse = Vec<Train>;

#[derive(Debug, Deserialize)]
pub struct Train {
    pub lat: String,
    pub lon: String,

    #[serde(rename = "trainno")]
    pub train_number: String,
    pub service: String,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub dest: RegionalRailStop,

    #[serde(rename = "currentstop", deserialize_with = "deserialize_string_enum")]
    pub current_stop: RegionalRailStop,

    #[serde(rename = "nextstop", deserialize_with = "deserialize_string_enum")]
    pub next_stop: RegionalRailStop,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub line: RegionalRailsLine,

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
