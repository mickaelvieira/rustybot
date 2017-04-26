use dom::text::HtmlString;
use dom::url::ParsedUrl;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Link {
    uri: ParsedUrl,
    value: String,
    title: String,
}

#[allow(dead_code)]
impl Link {
    pub fn new(url: &str, value: &str, title: &str) -> Link {
        Link {
            uri: ParsedUrl::new(url),
            value: String::from_html(value),
            title: String::from_html(title),
        }
    }
}
