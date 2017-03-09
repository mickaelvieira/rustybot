use dom::text::Text;
use dom::parsed_url::ParsedUrl;

#[allow(dead_code)]
pub struct Link {
    uri: ParsedUrl,
    value: Text,
    title: Text,
}

#[allow(dead_code)]
impl Link {
    pub fn new(url: &str, value: &str, title: &str) -> Link {
        Link {
            uri: ParsedUrl::new(url),
            value: Text::new(value),
            title: Text::new(title),
        }
    }
}
