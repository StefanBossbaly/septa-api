use crate::types::RegionalRailStop;

pub trait Request {
    fn into_params(self) -> Vec<(&'static str, String)>;
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Self::North => "N".to_string(),
            Self::South => "S".to_string(),
        }
    }
}

pub struct ArrivalsRequest {
    pub station: RegionalRailStop,
    pub results: Option<u8>,
    pub direction: Option<Direction>,
}

impl Request for ArrivalsRequest {
    fn into_params(self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();

        params.push(("station", self.station.to_string()));

        if let Some(direction) = self.direction {
            params.push(("direction", direction.to_string()));
        }

        if let Some(results) = self.results {
            params.push(("results", results.to_string()));
        }

        params
    }
}

pub struct NextToArriveRequest {
    pub starting_station: RegionalRailStop,
    pub ending_station: RegionalRailStop,
    pub results: Option<u8>,
}

impl Request for NextToArriveRequest {
    fn into_params(self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();

        params.push(("req1", self.starting_station.to_string()));
        params.push(("req2", self.ending_station.to_string()));

        if let Some(results) = self.results {
            params.push(("req3", results.to_string()));
        }

        params
    }
}
