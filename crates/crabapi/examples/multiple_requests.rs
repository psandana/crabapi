use crabapi::core::requests::{Url, build_request, send_requests};
use http::{HeaderMap, Method};
use reqwest::{Body, Client};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // send 10 requests
    let mut reqs = vec![];
    for i in 0..10 {
        let mut headers = HeaderMap::new();
        let query = vec![];
        headers.insert("key", i.to_string().parse().unwrap());
        reqs.push(build_request(
            &client,
            Url::parse("http://localhost:7878").unwrap(),
            query,
            Method::POST,
            headers,
            Body::from("POST Request"),
        ));
    }

    let handles = send_requests(reqs);
    for handle in handles {
        let body = handle.await??.text().await?;
        println!("body: {}", body.len());
    }

    Ok(())
}
