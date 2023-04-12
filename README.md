# Project Title

SEPTA API Async Rust Client.

## Project Description

This project provides developers with a standard and ergonomic Rust API for calling the various endpoints in
SEPTA API. It handles the serialization and deserialization of the requests and responses and allows the
developer to use the provided structures.

## API Status

### Real Time Data API

| Endpoint                     | Implemented | Tested |
| ---------------------------- | ----------- | ------ |
| `/Arrivals/index.php`        | ✅          | ✅     |
| `/TrainView/index.php`       | ✅          | ✅     |
| `/NextToArrive/index.php`    | ❌          | ❌     |
| `/TransitView/index.php`     | ❌          | ❌     |
| `/TransitViewAll/index.php`  | ❌          | ❌     |
| `/BusDetours/index.php`      | ❌          | ❌     |
| `/Alerts/index.php`          | ❌          | ❌     |
| `/Alerts/get_alert_data.php` | ❌          | ❌     |
| `/elevator/index.php`        | ❌          | ❌     |

### Static Data API

| Endpoint                       | Implemented | Tested |
| ------------------------------ | ----------- | ------ |
| `/RRSchedules/index.php`       | ❌          | ❌     |
| `/BusSchedules/index.php`      | ❌          | ❌     |
| `/Stops/index.php`             | ❌          | ❌     |
| `/locations/get_locations.php` | ❌          | ❌     |

## Authors

Stefan Bossbaly

## License

This project is licensed under the MIT License - see the LICENSE file for details

## Acknowledgments

- [SEPTA](https://www5.septa.org/)
- [SEPTA API](https://www3.septa.org/#/)
