#[macro_use]
extern crate lazy_static;

use septa_api::types::{RegionalRailStop, RegionalRailsLine};
use serde::{de::value::StrDeserializer, Deserialize};
use std::collections::{BTreeMap, BTreeSet};
use strum::IntoEnumIterator;

lazy_static! {
    static ref GTFS_DATA: gtfs_structures::Gtfs = {
        gtfs_structures::Gtfs::new(
            format!(
                "{}/tests/gtfs_data/septa_rail.zip",
                env!("CARGO_MANIFEST_DIR")
            )
            .as_str(),
        )
        .expect("Could not load GTFS data")
    };
}

#[test]
fn test_regional_rail_line_ids_test() -> Result<(), Box<dyn std::error::Error>> {
    let gtfs_rails = &GTFS_DATA;

    assert_eq!(gtfs_rails.routes.len(), RegionalRailsLine::iter().count());

    let gtfs_line_ids = gtfs_rails
        .routes
        .values()
        .map(|route| route.id.clone())
        .collect::<BTreeSet<String>>();

    let enum_line_ids = RegionalRailsLine::iter()
        .map(|line| line.id().to_string())
        .collect::<BTreeSet<String>>();

    assert_eq!(gtfs_line_ids, enum_line_ids);

    Ok(())
}

#[test]
fn test_regional_rail_name_test() -> Result<(), Box<dyn std::error::Error>> {
    let gtfs_rails = &GTFS_DATA;

    assert_eq!(
        gtfs_rails.stops.len(),
        RegionalRailStop::iter()
            .filter(|p| !matches!(p, RegionalRailStop::Unknown(_)))
            .count()
    );

    let gtfs_stop_names = gtfs_rails
        .stops
        .values()
        .map(|stop| {
            stop.name
                .as_ref()
                .expect("GTFS stop name should be populated")
                .to_owned()
        })
        .collect::<BTreeSet<String>>();

    let enum_stop_names = RegionalRailStop::iter()
        .filter(|p| !matches!(p, RegionalRailStop::Unknown(_)))
        .map(|stop| stop.to_string())
        .collect::<BTreeSet<String>>();

    for (gtfs_itr, enum_itr) in gtfs_stop_names.into_iter().zip(enum_stop_names) {
        assert_eq!(gtfs_itr, enum_itr);
    }

    Ok(())
}

#[test]
fn test_regional_rail_lat_long_test() -> Result<(), Box<dyn std::error::Error>> {
    let gtfs_rails = &GTFS_DATA;

    let gtfs_stop_names_to_lat_long = gtfs_rails
        .stops
        .values()
        .map(|stop| {
            (
                stop.name
                    .as_ref()
                    .expect("GTFS stop name should be populated")
                    .to_owned(),
                (
                    stop.latitude.expect("GTFS latitude should be populated"),
                    stop.longitude.expect("GTFS longitude should be populate"),
                ),
            )
        })
        .collect::<BTreeMap<String, (f64, f64)>>();

    let enum_stop_names_to_lat_long = RegionalRailStop::iter()
        .filter(|p| !matches!(p, RegionalRailStop::Unknown(_)))
        .map(|stop| (stop.to_string(), stop.lat_lon().unwrap()))
        .collect::<BTreeMap<String, (f64, f64)>>();

    for (gtfs_itr, enum_itr) in gtfs_stop_names_to_lat_long
        .into_iter()
        .zip(enum_stop_names_to_lat_long)
    {
        assert_eq!(gtfs_itr.0, enum_itr.0);
        assert_eq!(gtfs_itr.1, enum_itr.1);
    }

    Ok(())
}

#[test]
fn test_regional_rail_stop_id_test() -> Result<(), Box<dyn std::error::Error>> {
    let gtfs_rails = &GTFS_DATA;

    let gtfs_stop_names_to_stop_id = gtfs_rails
        .stops
        .values()
        .map(|stop| {
            (
                stop.name
                    .as_ref()
                    .expect("GTFS stop name should be populated")
                    .to_owned(),
                stop.id.parse::<u32>().unwrap(),
            )
        })
        .collect::<BTreeMap<String, u32>>();

    let enum_stop_names_to_stop_id = RegionalRailStop::iter()
        .filter(|p| !matches!(p, RegionalRailStop::Unknown(_)))
        .map(|stop| (stop.to_string(), stop.stop_id().unwrap()))
        .collect::<BTreeMap<String, u32>>();

    for (gtfs_itr, enum_itr) in gtfs_stop_names_to_stop_id
        .into_iter()
        .zip(enum_stop_names_to_stop_id)
    {
        assert_eq!(gtfs_itr.0, enum_itr.0);
        assert_eq!(gtfs_itr.1, enum_itr.1);
    }

    Ok(())
}

#[test]
fn test_deserialize_regional_rail() -> Result<(), Box<dyn std::error::Error>> {
    let gtfs_rails = &GTFS_DATA;

    gtfs_rails.stops.values().for_each(|stop| {
        let deserializer: StrDeserializer<'_, serde_json::error::Error> = StrDeserializer::new(
            stop.name
                .as_ref()
                .expect("GTFS stop name should be populated"),
        );
        RegionalRailStop::deserialize(deserializer).unwrap();
    });

    Ok(())
}
