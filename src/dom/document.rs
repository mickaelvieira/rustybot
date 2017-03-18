use dom::text::Text;
use dom::link::Link;
use dom::parsed_url::ParsedUrl;

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
