use crabapi::cli::Cli;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::new().run().await
}
