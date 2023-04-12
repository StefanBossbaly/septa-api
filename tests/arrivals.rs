use mockito::{Mock, ServerGuard};
use septa_api::{
    requests::{ArrivalsRequest, Direction},
    types::{RegionalRailStop, RegionalRailsLine, ServiceType},
    Client,
};

fn create_mock_server(server: &mut ServerGuard, endpoint: &str) -> Mock {
    server.mock("GET", endpoint)
}

#[tokio::test]
async fn test_deserialize1_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new();
    let mock_server = create_mock_server(
        &mut server,
        "/Arrivals/index.php?station=Temple+University&direction=N",
    )
    .with_body(
        r#"
        {
            "Temple U Departures: April 11, 2023, 6:29 pm": [
              {
                "Northbound": [
                  {
                    "direction": "N",
                    "path": "R5/3N",
                    "train_id": "5344",
                    "origin": "Thorndale",
                    "destination": "West Trenton",
                    "line": "West Trenton",
                    "status": "12 min",
                    "service_type": "LOCAL",
                    "next_station": "Suburban Station",
                    "sched_time": "2023-04-11 18:30:00.000",
                    "depart_time": "2023-04-11 18:30:00.000",
                    "track": "2",
                    "track_change": null,
                    "platform": "",
                    "platform_change": null
                  },
                  {
                    "direction": "N",
                    "path": "R8N",
                    "train_id": "9854",
                    "origin": "Chestnut Hill West",
                    "destination": "Temple U",
                    "line": "Fox Chase",
                    "status": "On Time",
                    "service_type": "LOCAL",
                    "next_station": "Temple U",
                    "sched_time": "2023-04-11 18:37:00.000",
                    "depart_time": "2023-04-11 18:37:00.000",
                    "track": "1",
                    "track_change": null,
                    "platform": "",
                    "platform_change": null
                  },
                  {
                    "direction": "N",
                    "path": "R4/8N",
                    "train_id": "4854",
                    "origin": "Airport Terminal E-F",
                    "destination": "Fox Chase",
                    "line": "Fox Chase",
                    "status": "On Time",
                    "service_type": "LOCAL",
                    "next_station": "30th Street Gray",
                    "sched_time": "2023-04-11 18:45:00.000",
                    "depart_time": "2023-04-11 18:45:00.000",
                    "track": "1",
                    "track_change": null,
                    "platform": "",
                    "platform_change": null
                  },
                  {
                    "direction": "N",
                    "path": "R7N",
                    "train_id": "9754",
                    "origin": "Trenton",
                    "destination": "Temple U",
                    "line": "Trenton",
                    "status": "On Time",
                    "service_type": "LOCAL",
                    "next_station": "North Philadelphia",
                    "sched_time": "2023-04-11 18:59:00.000",
                    "depart_time": "2023-04-11 18:59:00.000",
                    "track": "2",
                    "track_change": null,
                    "platform": "",
                    "platform_change": null
                  },
                  {
                    "direction": "N",
                    "path": "R3/2N",
                    "train_id": "3230",
                    "origin": "Wawa",
                    "destination": "Norristown",
                    "line": "Media/Wawa",
                    "status": "1 min",
                    "service_type": "LOCAL",
                    "next_station": "Secane",
                    "sched_time": "2023-04-11 19:06:00.000",
                    "depart_time": "2023-04-11 19:06:00.000",
                    "track": "1",
                    "track_change": null,
                    "platform": "",
                    "platform_change": null
                  }
                ]
              }
            ]
          }
    "#,
    )
    .create_async()
    .await;

    let client = Client::with_base_url(server.url().as_str());
    let arrivals_request = ArrivalsRequest {
        station: RegionalRailStop::TempleUniversity,
        results: None,
        direction: Some(Direction::North),
    };
    let arrival_response = client.arrivals(arrivals_request).await?;

    assert_eq!(
        arrival_response.title,
        "Temple U Departures: April 11, 2023, 6:29 pm"
    );
    assert_eq!(arrival_response.northbound.len(), 5);

    // Check the first arrival
    let arrival1 = &arrival_response.northbound[0];
    assert_eq!(arrival1.direction, "N");
    assert_eq!(arrival1.path, "R5/3N");
    assert_eq!(arrival1.train_id, "5344");
    assert_eq!(arrival1.origin, RegionalRailStop::Thorndale);
    assert_eq!(arrival1.destination, RegionalRailStop::WestTrenton);
    assert_eq!(arrival1.line, RegionalRailsLine::WestTrenton);
    assert_eq!(arrival1.status, "12 min");
    assert_eq!(arrival1.service_type, ServiceType::Local);
    assert_eq!(arrival1.next_station, RegionalRailStop::SuburbanStation);
    assert_eq!(arrival1.sched_time, "2023-04-11 18:30:00.000");
    assert_eq!(arrival1.depart_time, "2023-04-11 18:30:00.000");
    assert_eq!(arrival1.track, "2");
    assert_eq!(arrival1.track_change, None);
    assert_eq!(arrival1.platform, "");
    assert_eq!(arrival1.platform_change, None);

    // Check the second arrival
    let arrival2 = &arrival_response.northbound[1];
    assert_eq!(arrival2.direction, "N");
    assert_eq!(arrival2.path, "R8N");
    assert_eq!(arrival2.train_id, "9854");
    assert_eq!(arrival2.origin, RegionalRailStop::ChestnutHillWest);
    assert_eq!(arrival2.destination, RegionalRailStop::TempleUniversity);
    assert_eq!(arrival2.line, RegionalRailsLine::FoxChase);
    assert_eq!(arrival2.status, "On Time");
    assert_eq!(arrival2.service_type, ServiceType::Local);
    assert_eq!(arrival2.next_station, RegionalRailStop::TempleUniversity);
    assert_eq!(arrival2.sched_time, "2023-04-11 18:37:00.000");
    assert_eq!(arrival2.depart_time, "2023-04-11 18:37:00.000");
    assert_eq!(arrival2.track, "1");
    assert_eq!(arrival2.track_change, None);
    assert_eq!(arrival2.platform, "");
    assert_eq!(arrival2.platform_change, None);

    // Check the third arrival
    let arrival3 = &arrival_response.northbound[2];
    assert_eq!(arrival3.direction, "N");
    assert_eq!(arrival3.path, "R4/8N");
    assert_eq!(arrival3.train_id, "4854");
    assert_eq!(arrival3.origin, RegionalRailStop::AirportTerminalEF);
    assert_eq!(arrival3.destination, RegionalRailStop::FoxChase);
    assert_eq!(arrival3.line, RegionalRailsLine::FoxChase);
    assert_eq!(arrival3.status, "On Time");
    assert_eq!(arrival3.service_type, ServiceType::Local);
    assert_eq!(arrival3.next_station, RegionalRailStop::Gray30thStreet);
    assert_eq!(arrival3.sched_time, "2023-04-11 18:45:00.000");
    assert_eq!(arrival3.depart_time, "2023-04-11 18:45:00.000");
    assert_eq!(arrival3.track, "1");
    assert_eq!(arrival3.track_change, None);
    assert_eq!(arrival3.platform, "");
    assert_eq!(arrival3.platform_change, None);

    // Check the fourth arrival
    let arrival4 = &arrival_response.northbound[3];
    assert_eq!(arrival4.direction, "N");
    assert_eq!(arrival4.path, "R7N");
    assert_eq!(arrival4.train_id, "9754");
    assert_eq!(arrival4.origin, RegionalRailStop::Trenton);
    assert_eq!(arrival4.destination, RegionalRailStop::TempleUniversity);
    assert_eq!(arrival4.line, RegionalRailsLine::Trenton);
    assert_eq!(arrival4.status, "On Time");
    assert_eq!(arrival4.service_type, ServiceType::Local);
    assert_eq!(arrival4.next_station, RegionalRailStop::NorthPhiladelphia);
    assert_eq!(arrival4.sched_time, "2023-04-11 18:59:00.000");
    assert_eq!(arrival4.depart_time, "2023-04-11 18:59:00.000");
    assert_eq!(arrival4.track, "2");
    assert_eq!(arrival4.track_change, None);
    assert_eq!(arrival4.platform, "");
    assert_eq!(arrival4.platform_change, None);

    // Check the fifth arrival
    let arrival5 = &arrival_response.northbound[4];
    assert_eq!(arrival5.direction, "N");
    assert_eq!(arrival5.path, "R3/2N");
    assert_eq!(arrival5.train_id, "3230");
    assert_eq!(arrival5.origin, RegionalRailStop::Wawa);
    assert_eq!(arrival5.destination, RegionalRailStop::NorristownTC);
    assert_eq!(arrival5.line, RegionalRailsLine::MediaWawa);
    assert_eq!(arrival5.status, "1 min");
    assert_eq!(arrival5.service_type, ServiceType::Local);
    assert_eq!(arrival5.next_station, RegionalRailStop::Secane);
    assert_eq!(arrival5.sched_time, "2023-04-11 19:06:00.000");
    assert_eq!(arrival5.depart_time, "2023-04-11 19:06:00.000");
    assert_eq!(arrival5.track, "1");
    assert_eq!(arrival5.track_change, None);
    assert_eq!(arrival5.platform, "");
    assert_eq!(arrival5.platform_change, None);

    mock_server.assert_async().await;

    Ok(())
}
