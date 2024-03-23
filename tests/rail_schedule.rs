use chrono::NaiveTime;
use mockito::{Mock, ServerGuard};
use septa_api::{requests::RailScheduleRequest, types::RegionalRailStop, Client};

fn create_mock_server(server: &mut ServerGuard, endpoint: &str) -> Mock {
    server.mock("GET", endpoint)
}

#[tokio::test]
async fn test_deserialize1_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new_async().await;
    let mock_server = create_mock_server(&mut server, "/RRSchedules/index.php?req1=3236")
        .with_body(
            r#"[
    {
        "station": "Wawa",
        "sched_tm": "9:08 pm",
        "est_tm": "9:09 pm",
        "act_tm": "9:09 pm"
    },
    {
        "station": "Elwyn Station",
        "sched_tm": "9:13 pm",
        "est_tm": "9:14 pm",
        "act_tm": "9:14 pm"
    },
    {
        "station": "Media",
        "sched_tm": "9:16 pm",
        "est_tm": "9:17 pm",
        "act_tm": "9:17 pm"
    },
    {
        "station": "Moylan-Rose Valley",
        "sched_tm": "9:18 pm",
        "est_tm": "9:18 pm",
        "act_tm": "9:18 pm"
    },
    {
        "station": "Wallingford",
        "sched_tm": "9:20 pm",
        "est_tm": "9:20 pm",
        "act_tm": "9:20 pm"
    },
    {
        "station": "Swarthmore",
        "sched_tm": "9:23 pm",
        "est_tm": "9:22 pm",
        "act_tm": "9:22 pm"
    },
    {
        "station": "Morton",
        "sched_tm": "9:26 pm",
        "est_tm": "9:25 pm",
        "act_tm": "9:25 pm"
    },
    {
        "station": "Secane",
        "sched_tm": "9:29 pm",
        "est_tm": "9:27 pm",
        "act_tm": "9:27 pm"
    },
    {
        "station": "Primos",
        "sched_tm": "9:31 pm",
        "est_tm": "9:29 pm",
        "act_tm": "9:29 pm"
    },
    {
        "station": "Clifton-Aldan",
        "sched_tm": "9:33 pm",
        "est_tm": "9:31 pm",
        "act_tm": "9:31 pm"
    },
    {
        "station": "Gladstone",
        "sched_tm": "9:35 pm",
        "est_tm": "9:33 pm",
        "act_tm": "9:33 pm"
    },
    {
        "station": "Lansdowne",
        "sched_tm": "9:37 pm",
        "est_tm": "9:35 pm",
        "act_tm": "9:35 pm"
    },
    {
        "station": "Fernwood-Yeadon",
        "sched_tm": "9:39 pm",
        "est_tm": "9:37 pm",
        "act_tm": "9:37 pm"
    },
    {
        "station": "Angora",
        "sched_tm": "9:41 pm",
        "est_tm": "9:40 pm",
        "act_tm": "na"
    },
    {
        "station": "49th Street",
        "sched_tm": "9:44 pm",
        "est_tm": "9:43 pm",
        "act_tm": "na"
    },
    {
        "station": "Penn Medicine Station",
        "sched_tm": "9:49 pm",
        "est_tm": "9:48 pm",
        "act_tm": "na"
    },
    {
        "station": "Gray 30th Street",
        "sched_tm": "9:52 pm",
        "est_tm": "9:50 pm",
        "act_tm": "na"
    },
    {
        "station": "Suburban Station",
        "sched_tm": "9:57 pm",
        "est_tm": "9:55 pm",
        "act_tm": "na"
    },
    {
        "station": "Jefferson Station",
        "sched_tm": "10:02 pm",
        "est_tm": "10:00 pm",
        "act_tm": "na"
    },
    {
        "station": "Temple University",
        "sched_tm": "10:06 pm",
        "est_tm": "10:05 pm",
        "act_tm": "na"
    },
    {
        "station": "North Broad",
        "sched_tm": "10:08 pm",
        "est_tm": "10:07 pm",
        "act_tm": "na"
    },
    {
        "station": "Allegheny",
        "sched_tm": "10:11 pm",
        "est_tm": "10:10 pm",
        "act_tm": "na"
    },
    {
        "station": "East Falls",
        "sched_tm": "10:13 pm",
        "est_tm": "10:12 pm",
        "act_tm": "na"
    },
    {
        "station": "Wissahickon",
        "sched_tm": "10:16 pm",
        "est_tm": "10:15 pm",
        "act_tm": "na"
    },
    {
        "station": "Manayunk",
        "sched_tm": "10:19 pm",
        "est_tm": "10:18 pm",
        "act_tm": "na"
    },
    {
        "station": "Ivy Ridge",
        "sched_tm": "10:21 pm",
        "est_tm": "10:20 pm",
        "act_tm": "na"
    },
    {
        "station": "Miquon",
        "sched_tm": "10:25 pm",
        "est_tm": "10:24 pm",
        "act_tm": "na"
    },
    {
        "station": "Spring Mill",
        "sched_tm": "10:28 pm",
        "est_tm": "10:27 pm",
        "act_tm": "na"
    },
    {
        "station": "Conshohocken",
        "sched_tm": "10:31 pm",
        "est_tm": "10:30 pm",
        "act_tm": "na"
    },
    {
        "station": "Norristown T.C.",
        "sched_tm": "10:40 pm",
        "est_tm": "10:39 pm",
        "act_tm": "na"
    },
    {
        "station": "Main Street",
        "sched_tm": "10:43 pm",
        "est_tm": "10:42 pm",
        "act_tm": "na"
    },
    {
        "station": "Norristown - Elm Street",
        "sched_tm": "10:47 pm",
        "est_tm": "10:46 pm",
        "act_tm": "na"
    }
    ]"#,
        )
        .create_async()
        .await;

    let client: Client = Client::with_base_url(server.url().as_str());
    let rail_schedule_request = RailScheduleRequest {
        train_number: "3236".to_string(),
    };

    let response = client.rail_schedule(rail_schedule_request).await?;

    assert_eq!(response.len(), 32);

    // First Stop
    assert_eq!(response[0].station, RegionalRailStop::Wawa);
    assert_eq!(
        response[0].scheduled_time,
        NaiveTime::from_hms_opt(21, 8, 0).unwrap()
    );
    assert_eq!(
        response[0].estimated_time,
        NaiveTime::from_hms_opt(21, 9, 0).unwrap()
    );
    assert_eq!(
        response[0].actual_time,
        Some(NaiveTime::from_hms_opt(21, 9, 0).unwrap())
    );

    // Second Stop
    assert_eq!(response[1].station, RegionalRailStop::Elwyn);
    assert_eq!(
        response[1].scheduled_time,
        NaiveTime::from_hms_opt(21, 13, 0).unwrap()
    );
    assert_eq!(
        response[1].estimated_time,
        NaiveTime::from_hms_opt(21, 14, 0).unwrap()
    );
    assert_eq!(
        response[1].actual_time,
        Some(NaiveTime::from_hms_opt(21, 14, 0).unwrap())
    );

    // Third Stop
    assert_eq!(response[2].station, RegionalRailStop::Media);
    assert_eq!(
        response[2].scheduled_time,
        NaiveTime::from_hms_opt(21, 16, 0).unwrap()
    );
    assert_eq!(
        response[2].estimated_time,
        NaiveTime::from_hms_opt(21, 17, 0).unwrap()
    );
    assert_eq!(
        response[2].actual_time,
        Some(NaiveTime::from_hms_opt(21, 17, 0).unwrap())
    );

    // Fourth Stop
    assert_eq!(response[3].station, RegionalRailStop::MoylanRoseValley);
    assert_eq!(
        response[3].scheduled_time,
        NaiveTime::from_hms_opt(21, 18, 0).unwrap()
    );
    assert_eq!(
        response[3].estimated_time,
        NaiveTime::from_hms_opt(21, 18, 0).unwrap()
    );
    assert_eq!(
        response[3].actual_time,
        Some(NaiveTime::from_hms_opt(21, 18, 0).unwrap())
    );

    // Fifth Stop
    assert_eq!(response[4].station, RegionalRailStop::Wallingford);
    assert_eq!(
        response[4].scheduled_time,
        NaiveTime::from_hms_opt(21, 20, 0).unwrap()
    );
    assert_eq!(
        response[4].estimated_time,
        NaiveTime::from_hms_opt(21, 20, 0).unwrap()
    );
    assert_eq!(
        response[4].actual_time,
        Some(NaiveTime::from_hms_opt(21, 20, 0).unwrap())
    );

    // Sixth Stop
    assert_eq!(response[5].station, RegionalRailStop::Swarthmore);
    assert_eq!(
        response[5].scheduled_time,
        NaiveTime::from_hms_opt(21, 23, 0).unwrap()
    );
    assert_eq!(
        response[5].estimated_time,
        NaiveTime::from_hms_opt(21, 22, 0).unwrap()
    );
    assert_eq!(
        response[5].actual_time,
        Some(NaiveTime::from_hms_opt(21, 22, 0).unwrap())
    );

    // Last stop
    assert_eq!(response[31].station, RegionalRailStop::NorristownElmStreet);
    assert_eq!(
        response[31].scheduled_time,
        NaiveTime::from_hms_opt(22, 47, 0).unwrap()
    );
    assert_eq!(
        response[31].estimated_time,
        NaiveTime::from_hms_opt(22, 46, 0).unwrap()
    );
    assert_eq!(response[31].actual_time, None);

    mock_server.assert_async().await;

    Ok(())
}
