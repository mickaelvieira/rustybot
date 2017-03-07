use dom::text::Text;
use dom::link::Link;
use dom::parsedurl::ParsedUrl;

#[allow(dead_code)]
pub struct Body {
    content: String,
    headings: Vec<Heading>,
    links: Vec<Link>,
}

#[allow(dead_code)]
pub struct Head {
    title: String,
    charset: String,
    feeds: Vec<String>,
    twitter: String,
    facebook: String,
    language: String,
    description: String,
    canonical_url: ParsedUrl,
}

#[allow(dead_code)]
pub struct Document {
    body: Body,
    head: Head,
}

#[allow(dead_code)]
pub struct Heading {
    level: u32,
    value: Text,
}

impl Heading {
    pub fn from_tag(tag: &str, value: &str) -> Heading {
        let mut chars: Vec<char> = tag.to_string().chars().collect();
        let level: u32 = chars.last_mut()
            .unwrap_or(&mut '0')
            .to_digit(10)
            .unwrap_or(0);

        Heading{ level: level, value: Text::from(value) }
    }
}
