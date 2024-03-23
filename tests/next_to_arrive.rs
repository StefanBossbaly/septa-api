use chrono::NaiveTime;
use mockito::{Mock, ServerGuard};
use septa_api::{
    requests::NextToArriveRequest,
    types::{RegionalRailStop, RegionalRailsLine},
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
        "/NextToArrive/index.php?req1=Temple+University&req2=St.+Davids",
    )
    .with_body(
        r#"
    [
        {
            "orig_train": "2565",
            "orig_line": "Paoli/Thorndale",
            "orig_departure_time": "11:49PM",
            "orig_arrival_time": "12:33AM",
            "orig_delay": "On time",
            "isdirect": "true"
        }
    ]"#,
    )
    .create_async()
    .await;

    let client = Client::with_base_url(server.url().as_str());
    let next_to_arrive_request = NextToArriveRequest {
        starting_station: RegionalRailStop::TempleUniversity,
        ending_station: RegionalRailStop::StDavids,
        results: None,
    };
    let response = client.next_to_arrive(next_to_arrive_request).await?;

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].orig_train, "2565");
    assert_eq!(response[0].orig_line, RegionalRailsLine::PaoliThorndale);
    assert_eq!(
        response[0].orig_departure_time,
        NaiveTime::from_hms_opt(23, 49, 00).unwrap()
    );
    assert_eq!(
        response[0].orig_arrival_time,
        NaiveTime::from_hms_opt(0, 33, 00).unwrap()
    );
    assert_eq!(response[0].orig_delay, "On time");
    assert!(response[0].is_direct);

    mock_server.assert_async().await;

    Ok(())
}
