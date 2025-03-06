pub fn is_valid_url(url: &str) -> bool {
    super::Url::parse(url).is_ok()
}
