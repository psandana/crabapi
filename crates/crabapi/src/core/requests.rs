use reqwest::{Body, Client, Error, RequestBuilder, Response};
use std::collections::HashMap;
use tokio::task::JoinHandle;

pub use http::{HeaderMap, Method};
pub use reqwest::Url;

pub mod constants {
    use const_format::formatcp;
    use http::Method;

    pub const USER_AGENT: &str = formatcp!(
        "{} v{}",
        crate::core::app::constants::APP_NAME,
        crate::core::app::constants::APP_VERSION
    );

    pub const METHODS: [Method; 9] = [
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::HEAD,
        Method::OPTIONS,
        Method::CONNECT,
        Method::PATCH,
        Method::TRACE,
    ];

    pub const METHODS_STRING: [&str; 9] = [
        "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "CONNECT", "PATCH", "TRACE",
    ];

    pub const ALL_METHODS_AS_STRING: &str = "GET, POST, PUT, DELETE, HEAD, OPTIONS, CONNECT, PATCH, TRACE";
}

// TODO: Implement params too
pub fn build_request(
    client: &Client,
    url: Url,
    query: HashMap<String, String>,
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
