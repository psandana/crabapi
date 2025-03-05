use http::{HeaderMap, Method};
use reqwest::{Body, Client, Error, RequestBuilder, Response};
use tokio::task::JoinHandle;

pub fn build_request(
    client: &Client,
    url: &String,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> RequestBuilder {
    let request = reqwest::Request::new(method, url.parse().unwrap());
    RequestBuilder::from_parts(client.clone(), request)
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

// --------------
// -- EXAMPLE: --
// --------------
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new();
//
//     // send 1 request
//     let req = build_request(
//         &client,
//         "http://localhost:7878".to_string(),
//         Method::GET,
//         HeaderMap::new(),
//         Body::from("Get Request"),
//     );
//     let handles = send_requests(vec![req]);
//
//     for handle in handles {
//         let body = handle.await??.text().await?;
//         println!("body: {}", body.len());
//     }
//
//     // send 10 requests
//     let mut reqs = vec![];
//     for i in 0..10 {
//         let mut headers = HeaderMap::new();
//         headers.insert("key", i.to_string().parse().unwrap());
//         reqs.push(build_request(
//             &client,
//             "http://localhost:7878".to_string(),
//             Method::POST,
//             headers,
//             Body::from("POST Request"),
//         ));
//     }
//
//     let handles = send_requests(reqs);
//     for handle in handles {
//         let body = handle.await??.text().await?;
//         println!("body: {}", body.len());
//     }
//
//     Ok(())
// }
