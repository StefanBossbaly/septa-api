use geojson::{feature, Feature, FeatureCollection, Geometry, Value};
use septa_api::{types::RegionalRailStop, Client};
use std::{env, fs::File, io::Write};
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut features: Vec<Feature> = Vec::new();

    let client = Client::new();

    // Get all the active trains
    let response = client.train_view().await?;

    for train in response {
        features.push(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![train.lon, train.lat]))),
            id: None,
            properties: None,
            foreign_members: None,
        });
    }

    for station in RegionalRailStop::iter().filter(|p| !matches!(p, RegionalRailStop::Unknown(_))) {
        let lat_lon = station.lat_lon()?;

        features.push(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![lat_lon.1, lat_lon.0]))),
            id: Some(feature::Id::String(station.to_string())),
            properties: None,
            foreign_members: None,
        })
    }

    let gtfs_rails = gtfs_structures::Gtfs::new(
        format!(
            "{}/tests/gtfs_data/septa_rail.zip",
            env!("CARGO_MANIFEST_DIR")
        )
        .as_str(),
    )?;

    for (shape_name, shapes) in gtfs_rails.shapes.iter() {
        let line_string = geojson::Value::LineString(
            shapes
                .iter()
                .map(|x| vec![x.longitude, x.latitude])
                .collect(),
        );

        features.push(Feature {
            bbox: None,
            geometry: Some(Geometry {
                bbox: None,
                value: line_string,
                foreign_members: None,
            }),
            id: Some(feature::Id::String(shape_name.to_string())),
            properties: Some(serde_json::Map::from_iter([(
                "name".to_string(),
                serde_json::Value::String(shape_name.to_string()),
            )])),
            foreign_members: None,
        });
    }

    let json = geojson::GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    })
    .to_string();

    let dir = env::temp_dir();
    let mut file = File::create(dir.join("septa_geo.json"))?;
    file.write_all(json.as_bytes())?;

    println!("GeoJSON written to septa_geo.json in {}", dir.display());

    Ok(())
}
