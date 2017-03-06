extern crate url;

use std::convert::From;
// use std::convert::TryFrom;

use url::Url;

#[allow(dead_code)]
pub struct Head {
    title: String,
    charset: String,
    feeds: Vec<String>,
    twitter: String,
    facebook: String,
    language: String,
    description: String,
    canonical_url: String,
}

#[allow(dead_code)]
pub struct Body {
    content: String,
    headings: Vec<String>,
    links: Vec<String>,
}

#[allow(dead_code)]
pub struct Text {
    pub value: String
}

impl<'a> From<&'a str> for Text {
    fn from(text: &str) -> Text {
        Text { value: String::from(text) }
    }
}

#[allow(dead_code)]
pub struct ParsedUrl {
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub search: String,
    pub fragment: String,
}

impl<'a> From<&'a str> for ParsedUrl {

    fn from(url: &str) -> ParsedUrl {
        let parsed = Url::parse(url).unwrap();
        ParsedUrl {
            scheme: parsed.scheme().to_string(),
            username: parsed.username().to_string(),
            password: parsed.password().unwrap_or("").to_string(),
            host: parsed.host_str().unwrap_or("").to_string(),
            port: parsed.port().unwrap_or(80),
            path: parsed.path().to_string(),
            search: parsed.query().unwrap_or("").to_string(),
            fragment: parsed.fragment().unwrap_or("").to_string(),
        }
    }
}
