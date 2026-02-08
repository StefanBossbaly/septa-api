use crate::{errors, requests, responses};
use serde::de::DeserializeOwned;

const BASE_API_URL: &str = "https://www3.septa.org/api";

pub type Result<T> = std::result::Result<T, errors::Error>;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: BASE_API_URL.to_string(),
        }
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::ApiResponse<R>>()
            .await?;

        match response {
            responses::ApiResponse::Error(error) => Err(errors::Error::ApiErrorResponse(error)),
            responses::ApiResponse::Response(response) => Ok(response),
        }
    }

    async fn get_request<T: requests::Request, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        request: T,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = reqwest::Client::new()
            .get(url)
            .query(&request.into_params())
            .send()
            .await?
            .json::<responses::ApiResponse<R>>()
            .await?;

        match response {
            responses::ApiResponse::Error(error) => Err(errors::Error::ApiErrorResponse(error)),
            responses::ApiResponse::Response(response) => Ok(response),
        }
    }

    /// Returns a list of regional rail trains to arrive at a given station
    ///
    /// This function calls into the `/Arrivals/index.php` endpoint.
    ///
    /// The returned list is split into northbound and southbound trains. The definition of "Northbound" and "Southbound"
    /// is defined by SEPTA as the following:
    /// The direction are obviously not geographical references, but rather a reference to the old Reading and
    /// Pennsy Railroads. The key to understanding the direction is by using Suburban Station as a starting
    /// point: Any trains that move eastbound towards Market East are all considered Northbound; trains going
    /// from suburban to 30th St are all Southbound. The path field describes more accurately the path of travel
    /// along various branches.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing the request parameters
    ///
    /// # Example
    ///
    /// ```
    /// use septa_api::Client;
    /// use septa_api::requests::ArrivalsRequest;
    /// use septa_api::types::RegionalRailStop;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///     let request = ArrivalsRequest {
    ///         station: RegionalRailStop::SuburbanStation,
    ///         results: Some(5),
    ///         direction: None
    ///     };
    ///     let response = client.arrivals(request).await?;
    ///
    ///     // Loop through the northbound arrivals
    ///     for train in response.northbound {
    ///         println!("Train {} is currently {} on the {} line", train.train_id, train.status, train.line.map(|line| line.to_string()).unwrap_or("Unknown".to_string()));
    ///     }
    ///
    ///     // Loop through the southbound arrivals
    ///     for train in response.southbound {
    ///         println!("Train {} is currently {} on the {} line", train.train_id, train.status, train.line.map(|line| line.to_string()).unwrap_or("Unknown".to_string()));
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn arrivals(
        &self,
        request: requests::ArrivalsRequest,
    ) -> Result<responses::ArrivalsResponse> {
        self.get_request("/Arrivals/index.php", request).await
    }

    /// Returns a list of all active regional rail trains
    ///
    /// This function calls into the `/TrainView/index.php` endpoint.
    ///
    /// # Example
    ///
    /// ```
    /// use septa_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///     let response = client.train_view().await?;
    ///
    ///     for train in response {
    ///         println!("Train {} is currently {} mins late on the {} line", train.train_number, train.late, train.line.to_string());
    ///     }
    ///
    ///     Ok(())
    /// }
    pub async fn train_view(&self) -> Result<responses::TrainResponse> {
        self.get("/TrainView/index.php").await
    }

    /// Returns departure and arrival times between two different stations
    ///
    /// This function calls into the `/NextToArrive/index.php` endpoint.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing the request parameters
    ///
    /// # Example
    ///
    /// ```
    /// use septa_api::Client;
    /// use septa_api::requests::NextToArriveRequest;
    /// use septa_api::types::RegionalRailStop;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///     let request = NextToArriveRequest {
    ///         starting_station: RegionalRailStop::SuburbanStation,
    ///         ending_station: RegionalRailStop::Downingtown,
    ///         results: None,
    ///     };
    ///     let response = client.next_to_arrive(request).await?;
    ///
    ///     for next_to_arrive in response {
    ///        println!("Train {} is scheduled to arrive {}", next_to_arrive.orig_train, next_to_arrive.orig_departure_time);
    ///     }
    ///
    ///    Ok(())
    /// }
    pub async fn next_to_arrive(
        &self,
        request: requests::NextToArriveRequest,
    ) -> Result<responses::NextToArriveResponse> {
        self.get_request("/NextToArrive/index.php", request).await
    }

    /// Returns the schedule for a train by the train's number
    ///
    /// This function calls into the `/RRSchedules/index.php` endpoint.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing the request parameters
    ///
    /// # Example
    ///
    /// ```
    /// use septa_api::Client;
    /// use septa_api::requests::RailScheduleRequest;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///     let request = RailScheduleRequest {
    ///         train_number: "1729".to_string()
    ///     };
    ///     let response = client.rail_schedule(request).await?;
    ///
    ///     for schedule in response {
    ///        println!("The train is scheduled to arrive at {} at {}", schedule.station.to_string(), schedule.scheduled_time);
    ///     }
    ///
    ///    Ok(())
    /// }
    pub async fn rail_schedule(
        &self,
        request: requests::RailScheduleRequest,
    ) -> Result<responses::RailScheduleResponse> {
        self.get_request("/RRSchedules/index.php", request).await
    }
}
