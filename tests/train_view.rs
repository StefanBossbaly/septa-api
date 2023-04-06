use mockito::{Mock, ServerGuard};
use septa_api::{
    types::{RegionalRailStop, ServiceType},
    Client,
};

fn create_mock_server(server: &mut ServerGuard, endpoint: &str) -> Mock {
    server.mock("GET", endpoint)
}

#[tokio::test]
async fn test_deserialize1_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new();
    let mock_server = create_mock_server(&mut server, "/TrainView/index.php")
        .with_body(
            r#"
        [
            {
                "lat":"39.954174265",
                "lon":"-75.16763361",
                "trainno":"2333",
                "service":"LOCAL",
                "dest":"Wawa",
                "currentstop":"Suburban Station",
                "nextstop":"Gray 30th Street",
                "line":"Media\/Wawa",
                "consist":"872,871,858,857",
                "heading":189.8775840187919,
                "late":0,
                "SOURCE":"Norristown",
                "TRACK":"",
                "TRACK_CHANGE":""
            },
            {
                "lat":"40.200600166667",
                "lon":"-75.270441",
                "trainno":"2530",
                "service":"LOCAL",
                "dest":"Lansdale",
                "currentstop":"Gwynedd Valley",
                "nextstop":"North Wales",
                "line":"Lansdale\/Doylestown",
                "consist":"415,366,367,126,125",
                "heading":326.98421204774684,
                "late":0,
                "SOURCE":"Newark",
                "TRACK":"",
                "TRACK_CHANGE":""
            },
            {
                "lat":"39.953094545",
                "lon":"-75.162311045",
                "trainno":"3236",
                "service":"LOCAL",
                "dest":"Norristown",
                "currentstop":"Suburban Station",
                "nextstop":"Jefferson Station",
                "line":"Manayunk\/Norristown",
                "consist":"705,716,861,862",
                "heading":101.50453615740082,
                "late":0,
                "SOURCE":"Wawa",
                "TRACK":"1A",
                "TRACK_CHANGE":""
            }
        ]"#,
        )
        .create_async()
        .await;

    let client = Client::with_base_url(server.url().as_str());
    let trains = client.train_view().await?;

    assert_eq!(trains.len(), 3);
    assert_eq!(trains[0].lat, "39.954174265");
    assert_eq!(trains[0].lon, "-75.16763361");
    assert_eq!(trains[0].train_number, "2333");
    assert_eq!(trains[0].service, ServiceType::Local);
    assert_eq!(trains[0].dest, RegionalRailStop::Wawa);
    assert_eq!(trains[0].current_stop, RegionalRailStop::SuburbanStation);
    assert_eq!(trains[0].next_stop, RegionalRailStop::Gray30thStreet);
    assert_eq!(trains[0].consist, vec![872, 871, 858, 857]);
    assert_eq!(trains[0].heading, Some(189.8775840187919));
    assert_eq!(trains[0].late, 0);
    assert_eq!(trains[0].source, RegionalRailStop::NorristownTC);
    assert_eq!(trains[0].track, "");
    assert_eq!(trains[0].track_change, "");

    assert_eq!(trains[1].lat, "40.200600166667");
    assert_eq!(trains[1].lon, "-75.270441");
    assert_eq!(trains[1].train_number, "2530");
    assert_eq!(trains[1].service, ServiceType::Local);
    assert_eq!(trains[1].dest, RegionalRailStop::Lansdale);
    assert_eq!(trains[1].current_stop, RegionalRailStop::GwyneddValley);
    assert_eq!(trains[1].next_stop, RegionalRailStop::NorthWales);
    assert_eq!(trains[1].consist, vec![415, 366, 367, 126, 125]);
    assert_eq!(trains[1].heading, Some(326.98421204774684));
    assert_eq!(trains[1].late, 0);
    assert_eq!(trains[1].source, RegionalRailStop::Newark);
    assert_eq!(trains[1].track, "");
    assert_eq!(trains[1].track_change, "");

    assert_eq!(trains[2].lat, "39.953094545");
    assert_eq!(trains[2].lon, "-75.162311045");
    assert_eq!(trains[2].train_number, "3236");
    assert_eq!(trains[2].service, ServiceType::Local);
    assert_eq!(trains[2].dest, RegionalRailStop::NorristownTC);
    assert_eq!(trains[2].current_stop, RegionalRailStop::SuburbanStation);
    assert_eq!(trains[2].next_stop, RegionalRailStop::JeffersonStation);
    assert_eq!(trains[2].consist, vec![705, 716, 861, 862]);
    assert_eq!(trains[2].heading, Some(101.50453615740082));
    assert_eq!(trains[2].late, 0);
    assert_eq!(trains[2].source, RegionalRailStop::Wawa);
    assert_eq!(trains[2].track, "1A");
    assert_eq!(trains[2].track_change, "");

    mock_server.assert_async().await;

    Ok(())
}

// This is an actual response I got from the API when it was down
#[tokio::test]
async fn test_api_down_test_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new();
    let mock_server = create_mock_server(&mut server, "/TrainView/index.php")
        .with_body(
        r#"
        [
            {
                "error": "We apologize for the inconvenience, but we are experiencing difficulties at this time.  TrainView has been disabled."
            }
        ]"#)
        .create_async()
        .await;

    let client = Client::with_base_url(server.url().as_str());
    let trains = client.train_view().await;

    assert!(trains.is_err());
    match trains.unwrap_err() {
        septa_api::errors::Error::ApiErrorResponse(e) => assert_eq!(e, "We apologize for the inconvenience, but we are experiencing difficulties at this time.  TrainView has been disabled."),
        _ => unreachable!()
    }

    mock_server.assert_async().await;

    Ok(())
}
