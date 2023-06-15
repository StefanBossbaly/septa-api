use chrono::{NaiveDateTime, NaiveTime};
use serde::Deserialize;
use std::{collections::HashMap, convert::TryFrom};

use crate::{
    deserialize::{
        deserialize_api_error, deserialize_bool, deserialize_csv_encoded_string, deserialize_f64,
        deserialize_naive_date_time, deserialize_naive_time, deserialize_naive_time_with_space,
        deserialize_option_naive_time_with_space, deserialize_optional_string_enum,
        deserialize_string_enum,
    },
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

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "ArrivalsResponseBuilder")]
pub struct ArrivalsResponse {
    pub title: String,
    pub northbound: Vec<Arrivals>,
    pub southbound: Vec<Arrivals>,
}

#[derive(Debug, Deserialize)]
struct ArrivalsResponseBuilder(HashMap<String, Vec<serde_json::Value>>);

impl TryFrom<ArrivalsResponseBuilder> for ArrivalsResponse {
    type Error = String;

    fn try_from(builder: ArrivalsResponseBuilder) -> Result<Self, Self::Error> {
        if builder.0.len() != 1 {
            return Err(format!("expected 1 key, found {}", builder.0.len()));
        }

        let (title, values) = builder.0.into_iter().next().unwrap();

        let mut northbound = None;
        let mut southbound = None;

        for value in values.into_iter() {
            // SEPTA's API is inconsistent and will return an empty array when the are
            // no results.
            if let Ok(result) = serde_json::from_value::<Vec<serde_json::Value>>(value.clone()) {
                match result.len() {
                    0 => continue,
                    _ => {
                        return Err("Unknown response".to_string());
                    }
                }
            }

            let mut inner_values = serde_json::from_value::<HashMap<String, Vec<Arrivals>>>(value)
                .map_err(|e| e.to_string())?;

            if let Some(northbound_values) = inner_values.remove("Northbound") {
                if northbound.is_some() {
                    return Err("Found two northbound key values".to_string());
                }

                northbound = Some(northbound_values);
            }

            if let Some(southbound_values) = inner_values.remove("Southbound") {
                if southbound.is_some() {
                    return Err("Found two southbound key values".to_string());
                }

                southbound = Some(southbound_values);
            }
        }

        Ok(ArrivalsResponse {
            title,
            northbound: northbound.unwrap_or(Vec::new()),
            southbound: southbound.unwrap_or(Vec::new()),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
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

    #[serde(deserialize_with = "deserialize_optional_string_enum")]
    pub next_station: Option<RegionalRailStop>,

    #[serde(deserialize_with = "deserialize_naive_date_time")]
    pub sched_time: NaiveDateTime,

    #[serde(deserialize_with = "deserialize_naive_date_time")]
    pub depart_time: NaiveDateTime,
    pub track: String,
    pub track_change: Option<String>,
    pub platform: String,
    pub platform_change: Option<String>,
}

pub type TrainApiResponse = ApiResponse<TrainResponse>;
pub type TrainResponse = Vec<Train>;

#[derive(Debug, Deserialize, Clone)]
pub struct Train {
    #[serde(deserialize_with = "deserialize_f64")]
    pub lat: f64,

    #[serde(deserialize_with = "deserialize_f64")]
    pub lon: f64,

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

pub type NextToArriveApiResponse = ApiResponse<NextToArriveResponse>;
pub type NextToArriveResponse = Vec<NextToArrive>;

#[derive(Debug, Deserialize, Clone)]
pub struct NextToArrive {
    pub orig_train: String,

    #[serde(deserialize_with = "deserialize_string_enum")]
    pub orig_line: RegionalRailsLine,

    #[serde(deserialize_with = "deserialize_naive_time")]
    pub orig_departure_time: NaiveTime,

    #[serde(deserialize_with = "deserialize_naive_time")]
    pub orig_arrival_time: NaiveTime,

    pub orig_delay: String,

    #[serde(rename = "isdirect", deserialize_with = "deserialize_bool")]
    pub is_direct: bool,
}

pub type RailScheduleApiResponse = ApiResponse<RailScheduleResponse>;
pub type RailScheduleResponse = Vec<RailSchedule>;

#[derive(Debug, Deserialize, Clone)]
pub struct RailSchedule {
    #[serde(deserialize_with = "deserialize_string_enum")]
    pub station: RegionalRailStop,

    #[serde(
        rename = "sched_tm",
        deserialize_with = "deserialize_naive_time_with_space"
    )]
    pub scheduled_time: NaiveTime,

    #[serde(
        rename = "est_tm",
        deserialize_with = "deserialize_naive_time_with_space"
    )]
    pub estimated_time: NaiveTime,

    #[serde(
        rename = "act_tm",
        deserialize_with = "deserialize_option_naive_time_with_space"
    )]
    pub actual_time: Option<NaiveTime>,
}
