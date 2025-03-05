use http::{HeaderMap, Method};
use reqwest::{Body, Client, Error, RequestBuilder, Response};
use tokio::task::JoinHandle;

pub fn build_request(
    client: &Client,
    url: String,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> RequestBuilder {
    let request = reqwest::Request::new(method, url.parse().unwrap());
    reqwest::RequestBuilder::from_parts(client.clone(), request)
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
