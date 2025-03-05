// Recommendation: use http::Request. Already have headers/body. We need an abstraction that works with Request to make it easier to add Auth-z/Auth-n headers.
// https://docs.rs/http/latest/http/request/struct.Request.html
pub use http::Request;
pub mod requests;
