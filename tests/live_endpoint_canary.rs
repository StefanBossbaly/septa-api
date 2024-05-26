//! This file contains tests that hit the live SEPTA API endpoints and deserialize the responses.
//! They operate the same way as production code with the exception that they are run using the
//! `serde_path_to_error` crate so that if deserialization fails, the path to the error is printed
//! to the console, making is easier to figure out where the offending data is located. I wish that
//! API responses had consistent types, but it seems that SEPTA's API is inconsistent since it is
//! most likely programmed in Python, PHP or some other God-forsaken language that doesn't have
//! static types.
use reqwest::Client;
use septa_api::{
    requests::{self, Request},
    responses,
    types::RegionalRailStop,
};

const BASE_API_URL: &str = "https://www3.septa.org/api";

#[tokio::test]
async fn test_live_arrivals_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    const ENDPOINT: &str = "/Arrivals/index.php";

    let request = requests::ArrivalsRequest {
        station: RegionalRailStop::SuburbanStation,
        results: Some(10),
        direction: None,
    };

    let bytes = Client::new()
        .get(format!("{}{}", BASE_API_URL, ENDPOINT))
        .query(&request.into_params())
        .send()
        .await?
        .bytes()
        .await?;

    let deserialized = &mut serde_json::Deserializer::from_slice(bytes.as_ref());

    let result: Result<responses::ArrivalsResponse, _> =
        serde_path_to_error::deserialize(deserialized);

    if let Err(err) = result {
        let path = err.path().to_string();
        panic!(
            "Error deserializing ArrivalsResponse: {}: {}",
            path,
            std::str::from_utf8(bytes.as_ref()).unwrap_or("Failed to convert bytes to string")
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_live_train_view_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    const ENDPOINT: &str = "/TrainView/index.php";

    let bytes = Client::new()
        .get(format!("{}{}", BASE_API_URL, ENDPOINT))
        .send()
        .await?
        .bytes()
        .await?;

    let deserialized = &mut serde_json::Deserializer::from_slice(bytes.as_ref());

    let result: Result<responses::TrainResponse, _> =
        serde_path_to_error::deserialize(deserialized);

    if let Err(err) = result {
        let path = err.path().to_string();
        panic!(
            "Error deserializing TrainResponse: {}: {}",
            path,
            std::str::from_utf8(bytes.as_ref()).unwrap_or("Failed to convert bytes to string")
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_live_next_to_arrive_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    const ENDPOINT: &str = "/NextToArrive/index.php";

    let request = requests::NextToArriveRequest {
        starting_station: RegionalRailStop::SuburbanStation,
        ending_station: RegionalRailStop::Paoli,
        results: None,
    };

    let bytes = Client::new()
        .get(format!("{}{}", BASE_API_URL, ENDPOINT))
        .query(&request.into_params())
        .send()
        .await?
        .bytes()
        .await?;

    let deserialized = &mut serde_json::Deserializer::from_slice(bytes.as_ref());

    let result: Result<responses::NextToArriveResponse, _> =
        serde_path_to_error::deserialize(deserialized);

    if let Err(err) = result {
        let path = err.path().to_string();
        panic!(
            "Error deserializing NextToArriveResponse: {}: {}",
            path,
            std::str::from_utf8(bytes.as_ref()).unwrap_or("Failed to convert bytes to string")
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_live_rail_schedule_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    const ENDPOINT: &str = "/RRSchedules/index.php";

    let request = requests::RailScheduleRequest {
        train_number: "514".to_string(),
    };

    let bytes = Client::new()
        .get(format!("{}{}", BASE_API_URL, ENDPOINT))
        .query(&request.into_params())
        .send()
        .await?
        .bytes()
        .await?;

    let deserialized = &mut serde_json::Deserializer::from_slice(bytes.as_ref());

    let result: Result<responses::RailScheduleResponse, _> =
        serde_path_to_error::deserialize(deserialized);

    match result {
        Ok(schedule) => {
            if schedule.is_empty() {
                panic!("Expected at least one schedule entry, found none");
            }
        }
        Err(err) => {
            let path = err.path().to_string();
            panic!(
                "Error deserializing RailScheduleResponse: {}: {}",
                path,
                std::str::from_utf8(bytes.as_ref()).unwrap_or("Failed to convert bytes to string")
            );
        }
    }

    Ok(())
}
