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
    let mut server = mockito::Server::new_async().await;
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
                "heading":"189.8775840187919",
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
                "heading":"326.98421204774684",
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
                "heading":"101.50453615740082",
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
    assert_eq!(trains[0].lat, 39.954174265);
    assert_eq!(trains[0].lon, -75.16763361);
    assert_eq!(trains[0].train_number, "2333");
    assert_eq!(trains[0].service, ServiceType::Local);
    assert_eq!(trains[0].dest, RegionalRailStop::Wawa);
    assert_eq!(trains[0].current_stop, RegionalRailStop::SuburbanStation);
    assert_eq!(trains[0].next_stop, RegionalRailStop::Gray30thStreet);
    assert_eq!(trains[0].consist, Some(vec![872, 871, 858, 857]));
    assert_eq!(trains[0].heading, Some(189.8775840187919));
    assert_eq!(trains[0].late, 0);
    assert_eq!(trains[0].source, RegionalRailStop::NorristownTC);
    assert_eq!(trains[0].track, "");
    assert_eq!(trains[0].track_change, "");

    assert_eq!(trains[1].lat, 40.200600166667);
    assert_eq!(trains[1].lon, -75.270441);
    assert_eq!(trains[1].train_number, "2530");
    assert_eq!(trains[1].service, ServiceType::Local);
    assert_eq!(trains[1].dest, RegionalRailStop::Lansdale);
    assert_eq!(trains[1].current_stop, RegionalRailStop::GwyneddValley);
    assert_eq!(trains[1].next_stop, RegionalRailStop::NorthWales);
    assert_eq!(trains[1].consist, Some(vec![415, 366, 367, 126, 125]));
    assert_eq!(trains[1].heading, Some(326.98421204774684));
    assert_eq!(trains[1].late, 0);
    assert_eq!(trains[1].source, RegionalRailStop::Newark);
    assert_eq!(trains[1].track, "");
    assert_eq!(trains[1].track_change, "");

    assert_eq!(trains[2].lat, 39.953094545);
    assert_eq!(trains[2].lon, -75.162311045);
    assert_eq!(trains[2].train_number, "3236");
    assert_eq!(trains[2].service, ServiceType::Local);
    assert_eq!(trains[2].dest, RegionalRailStop::NorristownTC);
    assert_eq!(trains[2].current_stop, RegionalRailStop::SuburbanStation);
    assert_eq!(trains[2].next_stop, RegionalRailStop::JeffersonStation);
    assert_eq!(trains[2].consist, Some(vec![705, 716, 861, 862]));
    assert_eq!(trains[2].heading, Some(101.50453615740082));
    assert_eq!(trains[2].late, 0);
    assert_eq!(trains[2].source, RegionalRailStop::Wawa);
    assert_eq!(trains[2].track, "1A");
    assert_eq!(trains[2].track_change, "");

    mock_server.assert_async().await;

    Ok(())
}

// #[tokio::test]
// async fn test_enpoint_rush_hour_async() -> Result<(), septa_api::errors::Error> {
//     let mut server = mockito::Server::new_async().await;
//     let mock_server = create_mock_server(&mut server, "/TrainView/index.php")
//         .with_body(
//             r#"
//         [{"lat":"39.954848975","lon":"-75.171935695","trainno":"1085","service":"LOCAL","dest":"Cynwyd","currentstop":"Suburban Station","nextstop":"Gray 30th Street","line":"Cynwyd","consist":"721","heading":"279.74492662582446","late":0,"SOURCE":"Suburban Station","TRACK":"","TRACK_CHANGE":""},{"lat":"39.954174265","lon":"-75.16763361","trainno":"2325","service":"LOCAL","dest":"Wawa","currentstop":"Jefferson Station","nextstop":"Suburban Station","line":"Media\/Wawa","consist":"862,861,716,705","heading":"231.00642546593664","late":0,"SOURCE":"Norristown","TRACK":"4A","TRACK_CHANGE":""},{"lat":"40.2150365","lon":"-75.277429333333","trainno":"2522","service":"EXP TO FT WASH","dest":"Doylestown","currentstop":"North Wales","nextstop":"Pennbrook","line":"Lansdale\/Doylestown","consist":"709,737,859,860,720","heading":"347.7435587071233","late":0,"SOURCE":"Newark","TRACK":"","TRACK_CHANGE":""},{"lat":"39.953094545","lon":"-75.162311045","trainno":"3228","service":"LOCAL","dest":"Norristown","currentstop":"Suburban Station","nextstop":"Jefferson Station","line":"Manayunk\/Norristown","consist":"849,850,837,838","heading":"107.61274167549522","late":0,"SOURCE":"Wawa","TRACK":"1A","TRACK_CHANGE":""},{"lat":"40.101389666667","lon":"-75.153833833333","trainno":"3482","service":"LTD","dest":"Warminster","currentstop":"Glenside","nextstop":"Glenside","line":"Warminster","consist":"359,358,433,434","heading":"290.477188571293","late":0,"SOURCE":"Media","TRACK":"","TRACK_CHANGE":""},{"lat":"39.9679065","lon":"-75.193806","trainno":"3541","service":"EXP TO BRYN MAWR","dest":"Malvern","currentstop":"Gray 30th Street","nextstop":"Bryn Mawr","line":"Paoli\/Thorndale","consist":"851,852,843,844","heading":"289.19671947255176","late":0,"SOURCE":"West Trenton","TRACK":"","TRACK_CHANGE":""},{"lat":"40.192689833333","lon":"-74.889080666667","trainno":"3545","service":"LOCAL","dest":"Malvern","currentstop":"Yardley","nextstop":"Woodbourne","line":"West Trenton","consist":"298,404,288","heading":"222.58767616564785","late":0,"SOURCE":"West Trenton","TRACK":"","TRACK_CHANGE":""},{"lat":"40.149177666667","lon":"-75.109299333333","trainno":"448","service":"LOCAL","dest":"Warminster","currentstop":"Willow Grove","nextstop":"Hatboro","line":"Warminster","consist":"290,170,169,124,123","heading":"43.75451380130835","late":1,"SOURCE":"Airport","TRACK":"","TRACK_CHANGE":""},{"lat":"39.952435","lon":"-75.187678166667","trainno":"452","service":"LOCAL","dest":"Warminster","currentstop":"Penn Medicine Station","nextstop":"Gray 30th Street","line":"Warminster","consist":"409,309,308,287,294","heading":"8.882803512108921","late":0,"SOURCE":"Airport","TRACK":"","TRACK_CHANGE":""},{"lat":"40.070399","lon":"-75.127514833333","trainno":"457","service":"LOCAL","dest":"Airport","currentstop":"Elkins Park","nextstop":"Melrose Park","line":"Warminster","consist":"704,732,711,726","heading":"169.94172285604236","late":0,"SOURCE":"Glenside","TRACK":"","TRACK_CHANGE":""},{"lat":"40.018509","lon":"-75.161840666667","trainno":"4571","service":"LOCAL","dest":"Thorndale","currentstop":"Glenside","nextstop":"Temple University","line":"Warminster","consist":"416,128,127,316,317","heading":"187.53765269474974","late":0,"SOURCE":"Warminster","TRACK":"","TRACK_CHANGE":""},{"lat":"40.039915166667","lon":"-75.1136495","trainno":"4850","service":"LOCAL","dest":"Fox Chase","currentstop":"Olney","nextstop":"Lawndale","line":"Fox Chase","consist":"419,420,117,118","heading":"46.26235490285393","late":1,"SOURCE":"Airport","TRACK":"","TRACK_CHANGE":""},{"lat":"40.125338666667","lon":"-75.2002895","trainno":"5227","service":"LOCAL","dest":"Newark","currentstop":"Fort Washington","nextstop":"Oreland","line":"Lansdale\/Doylestown","consist":"882,881,840,839","heading":"138.46253586113403","late":0,"SOURCE":"Doylestown","TRACK":"","TRACK_CHANGE":""},{"lat":"40.116627","lon":"-75.067928166667","trainno":"5340","service":"LOCAL","dest":"West Trenton","currentstop":"Bethayres","nextstop":"Philmont","line":"West Trenton","consist":"829,830,833,834","heading":"74.72469743453985","late":1,"SOURCE":"Bryn Mawr","TRACK":"","TRACK_CHANGE":""},{"lat":"40.008391666667","lon":"-75.290365666667","trainno":"5344","service":"LOCAL","dest":"West Trenton","currentstop":"Ardmore","nextstop":"Wynnewood","line":"Paoli\/Thorndale","consist":"729,703,727,841,842","heading":"123.68616569661378","late":0,"SOURCE":"Thorndale","TRACK":"","TRACK_CHANGE":""},{"lat":"40.002976","lon":"-75.1597655","trainno":"5442","service":"LOCAL","dest":"Glenside","currentstop":"Temple University","nextstop":"Wayne Junction","line":"Warminster","consist":"807,808,805,806","heading":"345.9821232007067","late":0,"SOURCE":"Paoli","TRACK":"","TRACK_CHANGE":""},{"lat":"40.054197833333","lon":"-75.262877833333","trainno":"6256","service":"LOCAL","dest":"Norristown","currentstop":"Ivy Ridge","nextstop":"Miquon","line":"Manayunk\/Norristown","consist":"107,108,447,448,302","heading":"318.28024096298424","late":0,"SOURCE":"30th St","TRACK":"","TRACK_CHANGE":""},{"lat":"40.026731333333","lon":"-75.153310333333","trainno":"6336","service":"EXP TO JENKINTOWN","dest":"West Trenton","currentstop":"Temple University","nextstop":"Jenkintown Wyncote","line":"West Trenton","consist":"335,334,289,345,344","heading":"55.8313598281415","late":0,"SOURCE":"30th St","TRACK":"","TRACK_CHANGE":""},{"lat":"40.144803166667","lon":"-75.113755","trainno":"6483","service":"LOCAL","dest":"30th St","currentstop":"Hatboro","nextstop":"Willow Grove","line":"Warminster","consist":"158,157,286,406","heading":"205.55879800609316","late":1,"SOURCE":"Warminster","TRACK":"","TRACK_CHANGE":""},{"lat":"39.95373494","lon":"-75.165502485","trainno":"6524","service":"LOCAL","dest":"Doylestown","currentstop":"Suburban Station","nextstop":"Jefferson Station","line":"Lansdale\/Doylestown","consist":"455,456,111,112,161,162","heading":null,"late":0,"SOURCE":"Suburban Station","TRACK":"2B","TRACK_CHANGE":""},{"lat":"40.2635945","lon":"-75.260800666667","trainno":"6594","service":"LOCAL","dest":"Doylestown","currentstop":"Fortuna","nextstop":"Colmar","line":"Lansdale\/Doylestown","consist":"313,312,304,105,106,291","heading":"45.86251803206733","late":4,"SOURCE":"30th St","TRACK":"","TRACK_CHANGE":""},{"lat":"39.952383845","lon":"-75.157408715","trainno":"6598","service":"LOCAL","dest":"Doylestown","currentstop":"Jefferson Station","nextstop":"Temple University","line":"Lansdale\/Doylestown","consist":"415,366,367,126,125","heading":"100.70910319098522","late":0,"SOURCE":"30th St","TRACK":"","TRACK_CHANGE":""},{"lat":"39.95282972","lon":"-75.15885418","trainno":"6599","service":"LOCAL","dest":"30th St","currentstop":"Jefferson Station","nextstop":"Suburban Station","line":"Paoli\/Thorndale","consist":"444,443,338,339","heading":"260.4075479410412","late":1,"SOURCE":"Lansdale","TRACK":"4B","TRACK_CHANGE":""},{"lat":"40.079324333333","lon":"-75.203620166667","trainno":"6782","service":"LOCAL","dest":"Chestnut H East","currentstop":"Gravers","nextstop":"Chestnut Hill East","line":"Chestnut Hill East","consist":"333,332,369,368","heading":"319.8869879198231","late":0,"SOURCE":"30th St","TRACK":"","TRACK_CHANGE":""},{"lat":"39.955321833333","lon":"-75.176672","trainno":"722","service":"LOCAL","dest":"Chestnut H East","currentstop":"Gray 30th Street","nextstop":"Suburban Station","line":"Chestnut Hill East","consist":"801,802,877,878,725","heading":"99.38936660191865","late":9,"SOURCE":"Trenton","TRACK":"2B","TRACK_CHANGE":""},{"lat":"40.005148333333","lon":"-75.131120833333","trainno":"725","service":"LOCAL","dest":"Trenton","currentstop":"North Philadelphia Amtrak","nextstop":"Bridesburg","line":"Trenton","consist":"424,423,164,163","heading":"88.40322748135765","late":1,"SOURCE":"Chestnut H East","TRACK":"","TRACK_CHANGE":""},{"lat":"40.056551","lon":"-75.094388","trainno":"8353","service":"LOCAL","dest":"Media","currentstop":"Cheltenham","nextstop":"Lawndale","line":"Fox Chase","consist":"296,336,337,410","heading":"224.14480940064274","late":2,"SOURCE":"Fox Chase","TRACK":"","TRACK_CHANGE":""},{"lat":"39.95404739","lon":"-75.16766761","trainno":"8455","service":"LOCAL","dest":"Airport","currentstop":"Suburban Station","nextstop":"Gray 30th Street","line":"Airport","consist":"388,389,400,411","heading":"280.4173507224026","late":2,"SOURCE":"Fox Chase","TRACK":"","TRACK_CHANGE":""},{"lat":"39.900452666667","lon":"-75.2799345","trainno":"9224","service":"LOCAL","dest":"Temple U","currentstop":"Glenolden","nextstop":"Folcroft","line":"Wilmington\/Newark","consist":"707,731,875,876","heading":"60.806862984038105","late":0,"SOURCE":"Newark","TRACK":"","TRACK_CHANGE":""},{"lat":"39.8764555","lon":"-75.327777166667","trainno":"9225","service":"LTD","dest":"Newark","currentstop":"Ridley Park","nextstop":"Chester TC","line":"Wilmington\/Newark","consist":"911,2503,2556,2513,2559,2410","heading":"218.90766581720533","late":0,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"39.797922833333","lon":"-75.452008166667","trainno":"9241","service":"LOCAL","dest":"Wilmington","currentstop":"Claymont","nextstop":"Claymont","line":"Wilmington\/Newark","consist":"114,113,452,451","heading":"219.94227126873974","late":4,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"39.900734833333","lon":"-75.4585705","trainno":"9349","service":"EXP TO PRIMOS","dest":"Wawa","currentstop":"Elwyn Station","nextstop":"Wawa","line":"Media\/Wawa","consist":"913,2512,2501,2505,2554,2404","heading":"299.3564555374248","late":0,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"39.9376595","lon":"-75.269421166667","trainno":"9351","service":"LOCAL","dest":"Media","currentstop":"Fernwood-Yeadon","nextstop":"Lansdowne","line":"Media\/Wawa","consist":"836,835,856,855","heading":"258.5291237985969","late":3,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"40.0076945","lon":"-75.288735166667","trainno":"9553","service":"LOCAL","dest":"Malvern","currentstop":"Wynnewood","nextstop":"Ardmore","line":"Paoli\/Thorndale","consist":"166,165,132,131","heading":"304.1343010809636","late":2,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"40.019384833333","lon":"-75.622108333333","trainno":"9593","service":"EXP TO WAYNE","dest":"Thorndale","currentstop":"Exton","nextstop":"Whitford","line":"Paoli\/Thorndale","consist":"915,2516,2552,2504,2518,2405","heading":"251.42110103805015","late":5,"SOURCE":"Jefferson","TRACK":"","TRACK_CHANGE":""},{"lat":"40.046778166667","lon":"-75.431281833333","trainno":"9595","service":"EXP TO BRYN MAWR","dest":"Thorndale","currentstop":"Devon","nextstop":"Berwyn","line":"Paoli\/Thorndale","consist":"174,173,408,138,137,299","heading":"281.6132612371177","late":0,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"40.2157865","lon":"-74.7571895","trainno":"9754","service":"LOCAL","dest":"Temple U","currentstop":"Trenton","nextstop":"Levittown","line":"Trenton","consist":"295,141,142,449,450","heading":"228.4153551988902","late":0,"SOURCE":"Trenton","TRACK":"","TRACK_CHANGE":""},{"lat":"40.159732333333","lon":"-74.796009166667","trainno":"9761","service":"LOCAL","dest":"Trenton","currentstop":"Levittown","nextstop":"Trenton","line":"Trenton","consist":"872,871,858,857","heading":"34.71091357316857","late":0,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"39.975117166667","lon":"-75.195656666667","trainno":"9847","service":"LOCAL","dest":"Chestnut H West","currentstop":"Gray 30th Street","nextstop":"North Philadelphia Amtrak","line":"Chestnut Hill West","consist":"293,412,382,383","heading":"47.71873527010791","late":0,"SOURCE":"Temple U","TRACK":"","TRACK_CHANGE":""},{"lat":"40.051092333333","lon":"-75.191666666667","trainno":"9854","service":"LOCAL","dest":"Temple U","currentstop":"Carpenter","nextstop":"Carpenter","line":"Chestnut Hill West","consist":"425,426,139,140","heading":"166.83926488984162","late":0,"SOURCE":"Chestnut H West","TRACK":"","TRACK_CHANGE":""}]
//         "#
//     )
//     .create_async()
//     .await;

//     let client = Client::with_base_url(server.url().as_str());
//     let trains = client.train_view().await?;

//     assert_eq!(trains.len(), 40);
//     assert_eq!(trains[0].lat, 39.954848975);
//     assert_eq!(trains[0].lon, -75.171935695);
//     assert_eq!(trains[0].train_number, "1085");
//     assert_eq!(trains[0].dest, RegionalRailStop::Cynwyd);
//     assert_eq!(trains[0].current_stop, RegionalRailStop::SuburbanStation);
//     assert_eq!(trains[0].next_stop, RegionalRailStop::Gray30thStreet);
//     assert_eq!(trains[0].line, RegionalRailsLine::Cynwyd);
//     assert_eq!(trains[0].consist, Some(vec![721]));
//     assert_eq!(trains[0].heading, Some(279.74492662582446));
//     assert_eq!(trains[0].late, 0);
//     assert_eq!(trains[0].source, RegionalRailStop::SuburbanStation);
//     assert_eq!(trains[0].track, "");
//     assert_eq!(trains[0].track_change, "");

//     mock_server.assert_async().await;

//     Ok(())
// }

// This is an actual response I got from the API when it was down
#[tokio::test]
async fn test_api_down_test_async() -> Result<(), septa_api::errors::Error> {
    let mut server = mockito::Server::new_async().await;
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
