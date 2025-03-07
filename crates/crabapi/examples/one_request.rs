use crabapi::core::requests::{Url, build_request, send_requests};
use http::{HeaderMap, Method};
use reqwest::{Body, Client};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // send 1 request
    let req = build_request(
        &client,
        Url::parse("http://localhost:7878").unwrap(),
        vec![],
        Method::GET,
        HeaderMap::new(),
        Body::from("Get Request"),
    );
    let handles = send_requests(vec![req]);

    for handle in handles {
        let body = handle.await??.text().await?;
        println!("body: {}", body.len());
    }

    Ok(())
}
