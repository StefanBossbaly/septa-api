use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use mockito::{Mock, ServerGuard};
use septa_api::{
    requests::{self, ArrivalsRequest, Direction},
    types::{self, RegionalRailStop, RegionalRailsLine, ServiceType},
    Client,
};

fn create_mock_server(server: &mut ServerGuard, endpoint: &str) -> Mock {
    server.mock("GET", endpoint)
}

#[tokio::test]
async fn test_deserialize1_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new_async().await;
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
    assert_eq!(
        arrival1.next_station,
        Some(RegionalRailStop::SuburbanStation)
    );
    assert_eq!(
        arrival1.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 30, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival1.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 30, 0, 0).unwrap()
        )
    );
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
    assert_eq!(
        arrival2.next_station,
        Some(RegionalRailStop::TempleUniversity)
    );
    assert_eq!(
        arrival2.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 37, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival2.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 37, 0, 0).unwrap()
        )
    );
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
    assert_eq!(
        arrival3.next_station,
        Some(RegionalRailStop::Gray30thStreet)
    );
    assert_eq!(
        arrival3.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 45, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival3.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 45, 0, 0).unwrap()
        )
    );
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
    assert_eq!(
        arrival4.next_station,
        Some(RegionalRailStop::NorthPhiladelphia)
    );
    assert_eq!(
        arrival4.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 59, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival4.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 59, 0, 0).unwrap()
        )
    );
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
    assert_eq!(arrival5.next_station, Some(RegionalRailStop::Secane));
    assert_eq!(
        arrival5.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(19, 6, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival5.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 11).unwrap(),
            NaiveTime::from_hms_milli_opt(19, 6, 0, 0).unwrap()
        )
    );
    assert_eq!(arrival5.track, "1");
    assert_eq!(arrival5.track_change, None);
    assert_eq!(arrival5.platform, "");
    assert_eq!(arrival5.platform_change, None);

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_deserialize2_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new_async().await;
    let mock_server = create_mock_server(
        &mut server,
        "/Arrivals/index.php?station=Malvern&direction=N",
    )
    .with_body(
        r#"
        {
          "Malvern Departures: April 12, 2023, 5:46 pm":[
             {
                "Northbound":[
                   {
                      "direction":"N",
                      "path":"R5\/3N",
                      "train_id":"5348",
                      "origin":"Malvern",
                      "destination":"West Trenton",
                      "line":"Paoli\/Thorndale",
                      "status":"On Time",
                      "service_type":"LOCAL",
                      "next_station":null,
                      "sched_time":"2023-04-12 18:26:00.000",
                      "depart_time":"2023-04-12 18:26:00.000",
                      "track":"1",
                      "track_change":null,
                      "platform":"",
                      "platform_change":null
                   },
                   {
                      "direction":"N",
                      "path":"R5\/3N",
                      "train_id":"5352",
                      "origin":"Thorndale",
                      "destination":"West Trenton",
                      "line":"Paoli\/Thorndale",
                      "status":"On Time",
                      "service_type":"LOCAL",
                      "next_station":null,
                      "sched_time":"2023-04-12 19:26:00.000",
                      "depart_time":"2023-04-12 19:26:00.000",
                      "track":"1",
                      "track_change":null,
                      "platform":"",
                      "platform_change":null
                   },
                   {
                      "direction":"N",
                      "path":"R5\/3N",
                      "train_id":"5356",
                      "origin":"Thorndale",
                      "destination":"West Trenton",
                      "line":"Paoli\/Thorndale",
                      "status":"On Time",
                      "service_type":"LOCAL",
                      "next_station":null,
                      "sched_time":"2023-04-12 20:26:00.000",
                      "depart_time":"2023-04-12 20:26:00.000",
                      "track":"1",
                      "track_change":null,
                      "platform":"",
                      "platform_change":null
                   },
                   {
                      "direction":"N",
                      "path":"R5N",
                      "train_id":"9564",
                      "origin":"Malvern",
                      "destination":"Temple U",
                      "line":"Paoli\/Thorndale",
                      "status":"On Time",
                      "service_type":"LOCAL",
                      "next_station":null,
                      "sched_time":"2023-04-12 21:31:00.000",
                      "depart_time":"2023-04-12 21:31:00.000",
                      "track":"1",
                      "track_change":null,
                      "platform":"",
                      "platform_change":null
                   },
                   {
                      "direction":"N",
                      "path":"R5N",
                      "train_id":"9566",
                      "origin":"Thorndale",
                      "destination":"Temple U",
                      "line":"Paoli\/Thorndale",
                      "status":"On Time",
                      "service_type":"LOCAL",
                      "next_station":null,
                      "sched_time":"2023-04-12 22:41:00.000",
                      "depart_time":"2023-04-12 22:41:00.000",
                      "track":"1",
                      "track_change":null,
                      "platform":"",
                      "platform_change":null
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
        station: RegionalRailStop::Malvern,
        results: None,
        direction: Some(Direction::North),
    };
    let arrival_response = client.arrivals(arrivals_request).await?;

    assert_eq!(
        arrival_response.title,
        "Malvern Departures: April 12, 2023, 5:46 pm"
    );
    assert_eq!(arrival_response.northbound.len(), 5);

    // Check the first arrival
    let arrival1 = &arrival_response.northbound[0];
    assert_eq!(arrival1.direction, "N");
    assert_eq!(arrival1.path, "R5/3N");
    assert_eq!(arrival1.train_id, "5348");
    assert_eq!(arrival1.origin, RegionalRailStop::Malvern);
    assert_eq!(arrival1.destination, RegionalRailStop::WestTrenton);
    assert_eq!(arrival1.line, RegionalRailsLine::PaoliThorndale);
    assert_eq!(arrival1.status, "On Time");
    assert_eq!(arrival1.service_type, ServiceType::Local);
    assert_eq!(arrival1.next_station, None);
    assert_eq!(
        arrival1.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 26, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival1.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(18, 26, 0, 0).unwrap()
        )
    );
    assert_eq!(arrival1.track, "1");
    assert_eq!(arrival1.track_change, None);
    assert_eq!(arrival1.platform, "");
    assert_eq!(arrival1.platform_change, None);

    // Check the second arrival
    let arrival2 = &arrival_response.northbound[1];
    assert_eq!(arrival2.direction, "N");
    assert_eq!(arrival2.path, "R5/3N");
    assert_eq!(arrival2.train_id, "5352");
    assert_eq!(arrival2.origin, RegionalRailStop::Thorndale);
    assert_eq!(arrival2.destination, RegionalRailStop::WestTrenton);
    assert_eq!(arrival2.line, RegionalRailsLine::PaoliThorndale);
    assert_eq!(arrival2.status, "On Time");
    assert_eq!(arrival2.service_type, ServiceType::Local);
    assert_eq!(arrival2.next_station, None);
    assert_eq!(
        arrival2.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(19, 26, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival2.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(19, 26, 0, 0).unwrap()
        )
    );
    assert_eq!(arrival2.track, "1");
    assert_eq!(arrival2.track_change, None);
    assert_eq!(arrival2.platform, "");
    assert_eq!(arrival2.platform_change, None);

    // Check the third arrival
    let arrival3 = &arrival_response.northbound[2];
    assert_eq!(arrival3.direction, "N");
    assert_eq!(arrival3.path, "R5/3N");
    assert_eq!(arrival3.train_id, "5356");
    assert_eq!(arrival3.origin, RegionalRailStop::Thorndale);
    assert_eq!(arrival3.destination, RegionalRailStop::WestTrenton);
    assert_eq!(arrival3.line, RegionalRailsLine::PaoliThorndale);
    assert_eq!(arrival3.status, "On Time");
    assert_eq!(arrival3.service_type, ServiceType::Local);
    assert_eq!(arrival3.next_station, None);
    assert_eq!(
        arrival3.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(20, 26, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival3.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(20, 26, 0, 0).unwrap()
        )
    );
    assert_eq!(arrival3.track, "1");
    assert_eq!(arrival3.track_change, None);
    assert_eq!(arrival3.platform, "");
    assert_eq!(arrival3.platform_change, None);

    // Check the fourth arrival
    let arrival4 = &arrival_response.northbound[3];
    assert_eq!(arrival4.direction, "N");
    assert_eq!(arrival4.path, "R5N");
    assert_eq!(arrival4.train_id, "9564");
    assert_eq!(arrival4.origin, RegionalRailStop::Malvern);
    assert_eq!(arrival4.destination, RegionalRailStop::TempleUniversity);
    assert_eq!(arrival4.line, RegionalRailsLine::PaoliThorndale);
    assert_eq!(arrival4.status, "On Time");
    assert_eq!(arrival4.service_type, ServiceType::Local);
    assert_eq!(arrival4.next_station, None);
    assert_eq!(
        arrival4.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(21, 31, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival4.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(21, 31, 0, 0).unwrap()
        )
    );
    assert_eq!(arrival4.track, "1");
    assert_eq!(arrival4.track_change, None);
    assert_eq!(arrival4.platform, "");
    assert_eq!(arrival4.platform_change, None);

    // Check the fifth arrival
    let arrival5 = &arrival_response.northbound[4];
    assert_eq!(arrival5.direction, "N");
    assert_eq!(arrival5.path, "R5N");
    assert_eq!(arrival5.train_id, "9566");
    assert_eq!(arrival5.origin, RegionalRailStop::Thorndale);
    assert_eq!(arrival5.destination, RegionalRailStop::TempleUniversity);
    assert_eq!(arrival5.line, RegionalRailsLine::PaoliThorndale);
    assert_eq!(arrival5.status, "On Time");
    assert_eq!(arrival5.service_type, ServiceType::Local);
    assert_eq!(arrival5.next_station, None);
    assert_eq!(
        arrival5.sched_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(22, 41, 0, 0).unwrap()
        )
    );
    assert_eq!(
        arrival5.depart_time,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
            NaiveTime::from_hms_milli_opt(22, 41, 0, 0).unwrap()
        )
    );
    assert_eq!(arrival5.track, "1");
    assert_eq!(arrival5.track_change, None);
    assert_eq!(arrival5.platform, "");
    assert_eq!(arrival5.platform_change, None);

    assert_eq!(arrival_response.southbound.len(), 0);

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_deserialize3_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new_async().await;
    let mock_server = create_mock_server(
        &mut server,
        "/Arrivals/index.php?station=Temple+University&results=2",
    )
    .with_body(
        r#"{"Temple U Departures: June 14, 2023, 9:07 pm":[{"Northbound":[{"direction":"N","path":"R4N","train_id":"464","origin":"Airport Terminal E-F","destination":"Warminster","line":"Warminster","status":"On Time","service_type":"LOCAL","next_station":"Jefferson","sched_time":"2023-06-14 21:15:00.000","depart_time":"2023-06-14 21:15:00.000","track":"2","track_change":null,"platform":"","platform_change":null},{"direction":"N","path":"R2\/5N","train_id":"2530","origin":"Newark","destination":"Lansdale","line":"Lansdale\/Doylestown","status":"5 min","service_type":"LOCAL","next_station":"30th Street Gray","sched_time":"2023-06-14 21:18:00.000","depart_time":"2023-06-14 21:18:00.000","track":"1","track_change":null,"platform":"","platform_change":null}]},{"Southbound":[{"direction":"S","path":"R4S","train_id":"469","origin":"Warminster","destination":"Airport","line":"Airport","status":"On Time","service_type":"LOCAL","next_station":"Temple U","sched_time":"2023-06-14 21:13:00.000","depart_time":"2023-06-14 21:13:00.000","track":"3","track_change":null,"platform":"","platform_change":null},{"direction":"S","path":"R5S","train_id":"6535","origin":"Doylestown","destination":"30th St","line":"Lansdale\/Doylestown","status":"1 min","service_type":"LOCAL","next_station":"Jenkintown-Wyncote","sched_time":"2023-06-14 21:24:00.000","depart_time":"2023-06-14 21:24:00.000","track":"4","track_change":null,"platform":"","platform_change":null}]}]}"#,
    )
    .create_async()
    .await;

    let client = Client::with_base_url(server.url().as_str());
    let arrivals_request = ArrivalsRequest {
        station: RegionalRailStop::TempleUniversity,
        results: Some(2),
        direction: None,
    };
    let arrival_response = client.arrivals(arrivals_request).await?;

    assert_eq!(
        arrival_response.title,
        "Temple U Departures: June 14, 2023, 9:07 pm"
    );

    assert_eq!(arrival_response.northbound.len(), 2);
    assert_eq!(arrival_response.northbound[0].direction, "N");
    assert_eq!(arrival_response.northbound[1].direction, "N");

    assert_eq!(arrival_response.southbound.len(), 2);
    assert_eq!(arrival_response.southbound[0].direction, "S");
    assert_eq!(arrival_response.southbound[1].direction, "S");

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn empty_deserialize_test() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new_async().await;
    let mock_server =
        create_mock_server(&mut server, "/Arrivals/index.php?station=Temple+University")
            .with_body(r#"{"Temple U Departures: April 15, 2023, 1:23 am":[[],[]]}"#)
            .create_async()
            .await;

    let client = Client::with_base_url(server.url().as_str());
    let arrivals_request = requests::ArrivalsRequest {
        station: types::RegionalRailStop::TempleUniversity,
        results: None,
        direction: None,
    };
    let arrival_response = client.arrivals(arrivals_request).await?;

    assert_eq!(
        arrival_response.title,
        "Temple U Departures: April 15, 2023, 1:23 am"
    );
    assert_eq!(arrival_response.northbound.len(), 0);
    assert_eq!(arrival_response.southbound.len(), 0);

    mock_server.assert_async().await;

    Ok(())
}
