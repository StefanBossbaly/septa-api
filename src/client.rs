use serde::de::DeserializeOwned;

use crate::{errors, requests, responses};

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

    pub async fn arrivals(
        &self,
        request: requests::ArrivalsRequest,
    ) -> Result<responses::ArrivalsResponse> {
        self.get_request("/Arrivals/index.php", request).await
    }

    pub async fn train_view(&self) -> Result<responses::TrainResponse> {
        self.get("/TrainView/index.php").await
    }

    pub async fn next_to_arrive(
        &self,
        request: requests::NextToArriveRequest,
    ) -> Result<responses::NextToArriveResponse> {
        self.get_request("/NextToArrive/index.php", request).await
    }

    pub async fn rail_schedule(
        &self,
        request: requests::RailScheduleRequest,
    ) -> Result<responses::RailScheduleResponse> {
        self.get_request("/RRSchedules/index.php", request).await
    }
}
