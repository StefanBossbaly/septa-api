use chrono::{NaiveDateTime, NaiveTime};
use serde::{ser::Error, Deserialize};
use std::{collections::HashMap, convert::TryFrom};

use crate::{
    deserialize::{
        deserialize_api_error, deserialize_bool, deserialize_f64, deserialize_naive_date_time,
        deserialize_naive_time, deserialize_naive_time_with_space,
        deserialize_option_csv_encoded_string, deserialize_option_naive_time_with_space,
        deserialize_optional_f64, deserialize_optional_string_enum, deserialize_string_enum,
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
    type Error = serde_json::Error;

    fn try_from(builder: ArrivalsResponseBuilder) -> Result<Self, Self::Error> {
        let (title, values) = {
            let builder_len = builder.0.len();
            match builder.0.into_iter().next() {
                Some((title, value)) => (title, value),
                None => {
                    return Err(serde_json::Error::custom(format!(
                        "expected 1 key, found {}",
                        builder_len
                    )))
                }
            }
        };

        let mut northbound = None;
        let mut southbound = None;

        for value in values.into_iter() {
            // SEPTA's API is inconsistent and will return an empty array when the are
            // no results.
            if let Ok(result) = serde_json::from_value::<Vec<serde_json::Value>>(value.clone()) {
                match result.len() {
                    0 => continue,
                    _ => {
                        return Err(serde_json::Error::custom("Unknown response"));
                    }
                }
            }

            let mut inner_values = serde_json::from_value::<HashMap<String, Vec<Arrivals>>>(value)
                .map_err(|err| {
                    serde_json::Error::custom(format!("Could not parse inner values: {}", err))
                })?;

            if let Some(northbound_values) = inner_values.remove("Northbound") {
                northbound = match northbound {
                    Some(_) => {
                        return Err(serde_json::Error::custom("Found two northbound key values"))
                    }
                    None => Some(northbound_values),
                };
            }

            if let Some(southbound_values) = inner_values.remove("Southbound") {
                southbound = match southbound {
                    Some(_) => {
                        return Err(serde_json::Error::custom("Found two southbound key values"))
                    }
                    None => Some(southbound_values),
                };
            }
        }

        Ok(ArrivalsResponse {
            title,
            northbound: northbound.unwrap_or_default(),
            southbound: southbound.unwrap_or_default(),
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

    #[serde(deserialize_with = "deserialize_optional_string_enum")]
    pub line: Option<RegionalRailsLine>,
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

    #[serde(deserialize_with = "deserialize_option_csv_encoded_string")]
    pub consist: Option<Vec<i32>>,

    #[serde(deserialize_with = "deserialize_optional_f64")]
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
