use crabapi::gui::run_gui;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_gui();

    Ok(())
}
