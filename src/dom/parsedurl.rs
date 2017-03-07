use url::Url;
use std::convert::From;

#[allow(dead_code)]
pub struct ParsedUrl {
    parsed: Url,
}

impl<'a> From<&'a str> for ParsedUrl {
    fn from(url: &str) -> ParsedUrl {
        ParsedUrl { parsed: Url::parse(url).unwrap() }
    }
}

impl ToString for ParsedUrl {
    fn to_string(&self) -> String {
        self.parsed.as_str().to_string()
    }
}
