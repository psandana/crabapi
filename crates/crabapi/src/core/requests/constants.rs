
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

pub const ALL_METHODS_AS_STRING: &str =
    "GET, POST, PUT, DELETE, HEAD, OPTIONS, CONNECT, PATCH, TRACE";
