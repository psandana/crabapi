use crabapi::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::new().run().await
}
