use dom::text::Text;
use dom::link::Link;
use dom::feeds::Feed;
use dom::parsed_url::ParsedUrl;

#[derive(Debug)]
pub struct Body {
    pub content: String,
    pub headings: Vec<Heading>,
    pub links: Vec<Link>,
}

#[derive(Debug)]
pub struct Head {
    pub title: Text,
    pub charset: String,
    pub feeds: Vec<Feed>,
    pub twitter: Social,
    pub facebook: Social,
    pub language: String,
    pub description: String,
    pub canonical_url: ParsedUrl,
}

#[derive(Debug)]
pub struct Document {
    pub body: Body,
    pub head: Head,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Heading {
    level: u32,
    value: Text,
}

impl Heading {
    pub fn from_tag(tag: &str, value: &str) -> Heading {
        let mut chars: Vec<char> = tag.to_string()
            .chars()
            .collect();
        let level: u32 = chars.last_mut()
            .unwrap_or(&mut '0')
            .to_digit(10)
            .unwrap_or(0);

        Heading {
            level: level,
            value: Text::new(value),
        }
    }

    pub fn level(&self) -> u32 {
        self.level
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Social {
    title: String,
    description: String,
    image: String,
    url: ParsedUrl,
}

impl Social {
    pub fn new(title: &str, description: &str, image: &str, url: &str) -> Social {
        Social {
            title: title.to_string(),
            description: description.to_string(),
            image: image.to_string(),
            url: ParsedUrl::new(url),
        }
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }

    pub fn get_image(&self) -> &str {
        self.image.as_str()
    }

    pub fn get_url(&self) -> String {
        self.url.to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_a_heading_of_level_1() {
        let h = Heading::from_tag("h1", "The Title");
        assert_eq!(h.level(), 1);
    }

    #[test]
    fn it_creates_a_heading_of_level_2() {
        let h = Heading::from_tag("h2", "The Title");
        assert_eq!(h.level(), 2);
    }

    #[test]
    fn it_creates_a_heading_of_level_3() {
        let h = Heading::from_tag("h3", "The Title");
        assert_eq!(h.level(), 3);
    }

    #[test]
    fn it_creates_a_heading_of_level_4() {
        let h = Heading::from_tag("h4", "The Title");
        assert_eq!(h.level(), 4);
    }

    #[test]
    fn it_creates_a_heading_of_level_5() {
        let h = Heading::from_tag("h5", "The Title");
        assert_eq!(h.level(), 5);
    }

    #[test]
    fn it_creates_a_heading_of_level_6() {
        let h = Heading::from_tag("H6", "The Title");
        assert_eq!(h.level(), 6);
    }

    #[test]
    fn it_has_the_level_0_when_it_can_not_be_parsed() {
        let h = Heading::from_tag("", "The Title");
        assert_eq!(h.level(), 0);
    }
}
