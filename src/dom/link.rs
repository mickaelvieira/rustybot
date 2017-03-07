use dom::text::Text;
use dom::parsedurl::ParsedUrl;

#[allow(dead_code)]
pub struct Link {
    href: ParsedUrl,
    value: Text,
    title: Text,
}
