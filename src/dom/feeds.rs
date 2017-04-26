use dom::parsed_url::ParsedUrl;

/// http://www.jonathanturner.org/2016/02/down-the-rabbit-hole-with-traits.html

pub trait FeedType {
    fn is_atom(&self) -> bool;
    fn is_rss(&self) -> bool;
}

impl FeedType for String {
    fn is_atom(&self) -> bool {
        self == "application/atom+xml"
    }
    fn is_rss(&self) -> bool {
        self == "application/rss+xml"
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Feed {
    href: ParsedUrl,
    title: String,
    feed_type: String,
}

#[allow(dead_code)]
impl Feed {
    pub fn new(url: ParsedUrl, title: &str, feed_type: &str) -> Feed {
        Feed {
            href: url,
            title: title.to_string(),
            feed_type: feed_type.to_string(),
        }
    }
}

impl FeedType for Feed {
    fn is_rss(&self) -> bool {
        self.feed_type.is_rss()
    }

    fn is_atom(&self) -> bool {
        self.feed_type.is_atom()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let _ = Feed::new(ParsedUrl::new("http://google.com"),
                          "whatever",
                          "application/atom+xml");
    }

    #[test]
    fn it_knows_when_it_is_a_rss_feed() {
        let f = Feed::new(ParsedUrl::new("http://google.com"),
                          "whatever",
                          "application/rss+xml");
        assert!(f.is_rss());
        assert!(!f.is_atom());
    }

    #[test]
    fn it_knows_when_it_is_an_atom_feed() {
        let f = Feed::new(ParsedUrl::new("http://google.com"),
                          "whatever",
                          "application/atom+xml");
        assert!(!f.is_rss());
        assert!(f.is_atom());
    }
}
