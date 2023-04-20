use septa_api::{requests, types, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client.train_view().await?;
    dbg!(response);

    let arrivals_request = requests::ArrivalsRequest {
        station: types::RegionalRailStop::TempleUniversity,
        results: None,
        direction: Some(requests::Direction::North),
    };
    let arrivals = client.arrivals(arrivals_request).await?;
    dbg!(arrivals);

    let rail_schedule = client
        .rail_schedule(requests::RailScheduleRequest {
            train_number: "3236".to_owned(),
        })
        .await?;
    dbg!(rail_schedule);

    Ok(())
}
