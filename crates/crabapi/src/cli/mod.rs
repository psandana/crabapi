use clap::{Arg, Command};
use http::Method;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let method = Arg::new("method")
        .short('X')
        .long("method")
        .value_name("METHOD")
        .help("HTTP method(GET, POST)")
        .default_value("GET");

    let url = Arg::new("url").help("Request URL").required(true);

    let matches = Command::new("crabapi")
        .version("0.1.0")
        .author("Microsoft")
        .about("Postman analog")
        .arg(method)
        .arg(url)
        .get_matches();

    let option = matches.get_one::<String>("method")
        .unwrap()
        .parse::<Method>()?;
    let url = matches.get_one::<String>("url");

    println!("url: {:?}, option: {:?}", url, option);
    Ok(())
}
