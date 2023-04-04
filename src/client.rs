use serde::de::DeserializeOwned;

use crate::{
    errors,
    requests::{self, Request},
    responses::{self, ApiResponse},
};

const BASE_API_URL: &str = "https://www3.septa.org/api";

pub type Result<T> = std::result::Result<T, errors::Error>;

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
            .json::<ApiResponse<R>>()
            .await?;

        match response {
            responses::ApiResponse::Error(error) => Err(error.into()),
            responses::ApiResponse::Response(response) => Ok(response),
        }
    }

    async fn get_request<T: Request, R: DeserializeOwned>(
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
            .json::<ApiResponse<R>>()
            .await?;

        match response {
            responses::ApiResponse::Error(error) => Err(error.into()),
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
}
