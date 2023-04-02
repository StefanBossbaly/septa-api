use crate::responses::TrainResponse;

const BASE_API_URL: &str = "https://www3.septa.org/api";

#[derive(Default)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn train_view(&self) -> Result<TrainResponse, Box<dyn std::error::Error>> {
        let url = format!("{}{}", BASE_API_URL, "/TrainView/index.php");

        let response = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .json::<TrainResponse>()
            .await?;

        Ok(response)
    }
}
