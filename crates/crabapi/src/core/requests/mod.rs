pub mod constants;
pub mod validators;

use reqwest::{Body, Client, Error, RequestBuilder, Response};
use tokio::task::JoinHandle;

pub use http::{HeaderMap, Method};
pub use reqwest::Url;

// TODO: Implement params too
pub fn build_request(
    client: &Client,
    url: Url,
    query: Vec<(String, String)>,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> RequestBuilder {
    let mut default_headers = HeaderMap::new();
    default_headers.insert("User-Agent", constants::USER_AGENT.parse().unwrap());

    let request = reqwest::Request::new(method, url);
    RequestBuilder::from_parts(client.clone(), request)
        .query(&query)
        .headers(default_headers)
        .headers(headers.clone())
        .body(body)
}

pub fn send_requests(requests: Vec<RequestBuilder>) -> Vec<JoinHandle<Result<Response, Error>>> {
    let mut handles = vec![];
    for request in requests {
        handles.push(tokio::spawn(async move { request.send().await }));
    }

    handles
}

pub async fn print_response(handle: JoinHandle<Result<Response, Error>>) -> Result<(), Error> {
    let response = handle.await.unwrap()?;
    let headers = response.headers().clone();
    let url = response.url().clone();
    let status = response.status();
    let version = response.version();
    let body = response.text().await?;

    println!("{url} - {status} - {version:#?} - ");
    for header in headers.iter() {
        println!("\t{:#?}: {:#?}", header.0, header.1);
    }
    println!();
    if body.len() > 80 {
        println!("Body:\n{}\n...[truncated]", &body[0..79]);
    } else {
        println!("Body:\n{}", body);
    }

    Ok(())
}
