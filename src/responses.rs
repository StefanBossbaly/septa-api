use serde_derive::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{
    deserialize::{deserialize_api_error, deserialize_csv_encoded_string, deserialize_string_enum},
    types::{RegionalRailStop, RegionalRailsLine, ServiceType},
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Response(T),
    #[serde(deserialize_with = "deserialize_api_error")]
    Error(String),
}

pub type ArrivalsApiResponse = ApiResponse<ArrivalsResponse>;

#[derive(Debug, Deserialize)]
#[serde(try_from = "ArrivalsResponseBuilder")]
pub struct ArrivalsResponse {
    pub title: String,
    pub northbound: Option<Vec<Arrivals>>,
    pub southbound: Option<Vec<Arrivals>>,
}

#[derive(Debug, Deserialize)]
struct ArrivalsResponseBuilder(HashMap<String, Vec<HashMap<String, Vec<Arrivals>>>>);

impl TryFrom<ArrivalsResponseBuilder> for ArrivalsResponse {
    type Error = String;

    fn try_from(builder: ArrivalsResponseBuilder) -> Result<Self, Self::Error> {
        dbg!(&builder.0);
        if builder.0.len() != 1 {
            return Err(format!("expected 1 key, found {}", builder.0.len()));
        }

        let (title, mut values) = builder.0.into_iter().next().unwrap();

        if values.len() != 1 {
            return Err(format!("expected 1 value, found {}", values.len()));
        }

        let northbound: Option<Vec<Arrivals>> = values[0]
            .get_mut("Northbound")
            .map(|northbound| northbound.drain(..).collect());

        let southbound: Option<Vec<Arrivals>> = values[0]
            .get_mut("Southbound")
            .map(|southbound| southbound.drain(..).collect());

        Ok(ArrivalsResponse {
            title,
            northbound,
            southbound,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Arrivals {
    pub direction: String,
    pub path: String,
    pub train_id: String,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub origin: RegionalRailStop,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub destination: RegionalRailStop,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub line: RegionalRailsLine,
    pub status: String,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub service_type: ServiceType,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub next_station: RegionalRailStop,
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

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub service: ServiceType,

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

    #[serde(rename = "SOURCE", deserialize_with = "deserialize_string_enum")]
    pub source: RegionalRailStop,

    #[serde(rename = "TRACK")]
    pub track: String,

    #[serde(rename = "TRACK_CHANGE")]
    pub track_change: String,
}
