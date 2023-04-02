use septa_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.train_view().await?;

    println!("{:#?}", response);

    Ok(())
}
