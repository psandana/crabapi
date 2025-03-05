use crate::core::requests::{build_request, send_requests};
use clap::{Arg, Command};
use http::{HeaderMap, Method};
use reqwest::{Body, Client};
use std::error::Error;

pub struct Cli {
    client: Client,
    url_arg: Arg,
    method_arg: Arg,
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
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let matches = Command::new("crabapi")
            .version("0.1.0")
            .author("Microsoft")
            .about("Postman analog")
            .arg(self.url_arg)
            .arg(self.method_arg)
            .get_matches();

        let option = matches
            .get_one::<String>("method")
            .unwrap()
            .parse::<Method>()?;
        let url = matches.get_one::<String>("url").unwrap();

        let request = build_request(&self.client, url, option, HeaderMap::new(), Body::from(""));

        let handles = send_requests(vec![request]);
        for handle in handles {
            let text = handle.await??.text().await?;
            println!("body: {:?}", text);
        }
        Ok(())
    }
}
