use std::collections::BTreeSet;

use septa_api::types::RegionalRailStop;
use strum::IntoEnumIterator;

fn load_gtfs_rails() -> Result<gtfs_structures::Gtfs, gtfs_structures::Error> {
    gtfs_structures::Gtfs::new(
        format!(
            "{}/tests/gtfs_data/septa_rail.zip",
            env!("CARGO_MANIFEST_DIR")
        )
        .as_str(),
    )
}

#[test]
fn test_regional_rail_count_stops_test() -> Result<(), Box<dyn std::error::Error>> {
    let gtfs_rails = load_gtfs_rails()?;

    assert_eq!(
        gtfs_rails.stops.len(),
        RegionalRailStop::iter()
            .filter(|p| !matches!(p, RegionalRailStop::Unknown(_)))
            .count()
    );

    Ok(())
}

#[test]
fn test_regional_rail_name_test() -> Result<(), Box<dyn std::error::Error>> {
    let gtfs_rails = load_gtfs_rails()?;

    let gtfs_stop_names = gtfs_rails
        .stops
        .into_values()
        .map(|stop| stop.name.to_string())
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
