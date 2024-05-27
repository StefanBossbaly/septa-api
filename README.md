# SEPTA-API ![crates.io](https://img.shields.io/crates/v/septa-api.svg) [![](https://docs.rs/septa-api/badge.svg)](https://docs.rs/septa-api)

This project provides developers with a standard and ergonomic Rust API for
calling the various endpoints in SEPTA (Southeastern Pennsylvania Transportation
Authority) API. It handles the serialization and deserialization of the requests
and responses and allows the developer to use the provided well-defined data
types. It also handles some of the messy parts of the API (multiple
serializations per stop, quarky endpoint responses, multiple datetime formats,
etc).

## Example Usage

```rust
use septa_api::{requests, types, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client with the default url
    let client = Client::new();

    // Get a list of all active trains
    let train_view_response = client.train_view().await?;
    for train in train_view_response {
        println!("Train {} is heading to {}", train.train_number, train.dest);
    }

    // Make a request to get the next arrivals
    let arrivals_request = requests::ArrivalsRequest {
        station: types::RegionalRailStop::TempleUniversity,
        results: None,
        direction: None,
    };
    let arrivals_response = client.arrivals(arrivals_request).await?;

    println!("Trains coming in southbound:");
    for southbound_arrival in arrivals_response.southbound {
        println!(
            "Train {} is {}",
            southbound_arrival.train_id, southbound_arrival.status
        );
    }

    println!("Trains coming in northbound:");
    for northbound_arrival in arrivals_response.northbound {
        println!(
            "Train {} is {}",
            northbound_arrival.train_id, northbound_arrival.status
        );
    }

    Ok(())
}
```

## Testing

This crate is tested daily during the morning (8:30 AM EST) and evening (5:15 PM
EST) rush hour commutes and in the dead of the night (3 AM EST). You may wonder
why such a dichotomy, and the reason is to attempt to get data when the network
is at its peak and during a lull. SEPTA is not the best in outlining all their
API responses so some of this library is reversed engineered. By capturing both
these states, we hope to detect any breaking changes in SEPTA's API and can
therefore fix them ASAP.

## API Implementation and Testing Status

### Real Time Data API

| Endpoint                     | Implemented | Tested |
| ---------------------------- | ----------- | ------ |
| `/Arrivals/index.php`        | ✅          | ✅     |
| `/TrainView/index.php`       | ✅          | ✅     |
| `/NextToArrive/index.php`    | ✅          | ✅     |
| `/TransitView/index.php`     | ❌          | ❌     |
| `/TransitViewAll/index.php`  | ❌          | ❌     |
| `/BusDetours/index.php`      | ❌          | ❌     |
| `/Alerts/index.php`          | ❌          | ❌     |
| `/Alerts/get_alert_data.php` | ❌          | ❌     |
| `/elevator/index.php`        | ❌          | ❌     |

### Static Data API

| Endpoint                       | Implemented | Tested |
| ------------------------------ | ----------- | ------ |
| `/RRSchedules/index.php`       | ✅          | ✅     |
| `/BusSchedules/index.php`      | ❌          | ❌     |
| `/Stops/index.php`             | ❌          | ❌     |
| `/locations/get_locations.php` | ❌          | ❌     |

## Authors

Stefan Bossbaly

## License

This project is licensed under the MIT License - see the LICENSE file for
details

## Acknowledgments

- [SEPTA](https://www5.septa.org/)
- [SEPTA API](https://www3.septa.org/#/)
