use crate::core::app::constants;
use crate::core::requests::{Url, build_request, constants as requests_constants, print_response, send_requests};
use clap::{Arg, ArgAction, Command};
use const_format::formatcp;
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use reqwest::{Body, Client};
use std::collections::HashMap;
use std::error::Error;

pub struct Cli {
    client: Client,
    url_arg: Arg,
    query_arg: Arg,
    method_arg: Arg,
    headers_arg: Arg,
    gui_arg: Arg,
    body_arg: Arg,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            client: Client::new(),
            url_arg: Arg::new("url")
                .help("Request URL")
                .required_unless_present("gui"),
            query_arg: Arg::new("query")
                .short('Q')
                .long("query")
                .value_name("QUERY")
                .action(ArgAction::Append)
                .help("Request query"),
            method_arg: Arg::new("method")
                .short('X')
                .long("method")
                .value_name("METHOD")
                .help(formatcp!("HTTP method({})", requests_constants::ALL_METHODS_AS_STRING))
                .default_value("GET"),
            headers_arg: Arg::new("headers")
                .short('H')
                .long("header")
                .value_name("HEADERS")
                .action(ArgAction::Append)
                .help("List of headers in format 'Key: Value'"),
            gui_arg: Arg::new("gui")
                .short('g')
                .long("gui")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Flag: Run GUI"),
            body_arg: Arg::new("body")
                .short('d')
                .long("data")
                .value_name("BODY")
                .help("Request body (For POST, PUT, PATCH request)"),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let matches = Command::new(constants::APP_COMMAND_NAME)
            .version(constants::APP_VERSION)
            .author(constants::APP_AUTHOR)
            .about(formatcp!(
                "{} - {}",
                constants::APP_NAME,
                constants::APP_DESCRIPTION
            ))
            .arg(self.url_arg)
            .arg(self.query_arg)
            .arg(self.method_arg)
            .arg(self.headers_arg)
            .arg(self.gui_arg)
            .arg(self.body_arg)
            .get_matches();

        if matches.get_flag("gui") {
            crate::gui::run_gui();
            return Ok(());
        }

        let mut query = HashMap::new();
        if let Some(query_values) = matches.get_many::<String>("query") {
            for query_value in query_values {
                if let Some((key, value)) = query_value.split_once(": ") {
                    query.insert(key.to_string(), value.to_string());
                }
            }
        }

        let method = matches
            .get_one::<String>("method")
            .unwrap()
            .parse::<Method>()?;
        let url = matches.get_one::<String>("url").unwrap();
        let url = Url::parse(url).unwrap();

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
        let default_body = String::from("");
        let body = matches.get_one::<String>("body").unwrap_or(&default_body);

        let request = build_request(
            &self.client,
            url,
            query,
            method,
            headers,
            Body::from(body.to_string()),
        );

        println!("Send request: {:?}\n", request);

        let handles = send_requests(vec![request]);
        for handle in handles {
            print_response(handle).await?;
        }
        Ok(())
    }
}
