use crate::core::requests::{build_request, print_response, send_requests};
use clap::{Arg, ArgAction, Command};
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use reqwest::{Body, Client};
use std::error::Error;

pub struct Cli {
    client: Client,
    url_arg: Arg,
    method_arg: Arg,
    headers_arg: Arg,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            client: Client::new(),
            url_arg: Arg::new("url").help("Request URL").required(true),
            method_arg: Arg::new("method")
                .short('X')
                .long("method")
                .value_name("METHOD")
                .help("HTTP method(GET, POST)")
                .default_value("GET"),
            headers_arg: Arg::new("headers")
                .short('H')
                .long("header")
                .value_name("HEADERS")
                .action(ArgAction::Append)
                .help("List of headers in format 'Key: Value'"),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let matches = Command::new("crabapi")
            .version("0.1.0")
            .author("Microsoft")
            .about("Postman analog")
            .arg(self.url_arg)
            .arg(self.method_arg)
            .arg(self.headers_arg)
            .get_matches();

        let option = matches
            .get_one::<String>("method")
            .unwrap()
            .parse::<Method>()?;
        let url = matches.get_one::<String>("url").unwrap();

        let mut headers = HeaderMap::new();
        if let Some(header_values) = matches.get_many::<String>("headers") {
            for header_value in header_values {
                if let Some((key, value)) = header_value.split_once(": ") {
                    headers.insert(
                        HeaderName::from_bytes(key.trim().as_bytes())?,
                        HeaderValue::from_str(value.trim())?,
                    );
                }
            }
        }

        let request = build_request(&self.client, url, option, headers, Body::from(""));

        println!("Send request: {:?}\n", request);

        let handles = send_requests(vec![request]);
        for handle in handles {
            print_response(handle).await?;
        }
        Ok(())
    }
}
