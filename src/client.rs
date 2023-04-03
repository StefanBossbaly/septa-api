use crate::{
    requests::{self, Request},
    responses::{self, ArrivalsResponse, TrainApiResponse, TrainResponse},
};

const BASE_API_URL: &str = "https://www3.septa.org/api";

#[derive(Default)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn arrivals(
        &self,
        request: requests::ArrivalsRequest,
    ) -> Result<responses::ArrivalsResponse, Box<dyn std::error::Error>> {
        let url = format!("{}{}", BASE_API_URL, "/Arrivals/index.php");

        let response = reqwest::Client::new()
            .get(&url)
            .query(&request.into_params())
            .send()
            .await?
            .json::<ArrivalsResponse>()
            .await?;

        Ok(response)
    }

    pub async fn train_view(&self) -> Result<TrainResponse, Box<dyn std::error::Error>> {
        let url = format!("{}{}", BASE_API_URL, "/TrainView/index.php");

        let response = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .json::<TrainApiResponse>()
            .await?;

        match response {
            responses::ApiResponse::Error(error) => Err(error.error.into()),
            responses::ApiResponse::Response(response) => Ok(response),
        }
    }
}
