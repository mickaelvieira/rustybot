use dom::parsed_url::ParsedUrl;

/// http://www.jonathanturner.org/2016/02/down-the-rabbit-hole-with-traits.html
#[allow(dead_code)]
#[derive(Debug)]
pub struct Feed {
    href: ParsedUrl,
    title: String,
    feed_type: String,
}

#[allow(dead_code)]
impl Feed {
    pub fn new(url: &str, title: &str, feed_type: &str) -> Feed {
        Feed {
            href: ParsedUrl::new(url),
            title: title.to_string(),
            feed_type: feed_type.to_string(),
        }
    }

    pub fn is_rss(&self) -> bool {
        self.feed_type.as_str() == "application/rss+xml"
    }

    pub fn is_atom(&self) -> bool {
        self.feed_type.as_str() == "application/atom+xml"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let _ = Feed::new("http://google.com", "whatever", "application/atom+xml");
    }

    #[test]
    fn it_knows_when_it_is_a_rss_feed() {
        let f = Feed::new("http://google.com", "whatever", "application/rss+xml");
        assert!(f.is_rss());
        assert!(!f.is_atom());
    }

    #[test]
    fn it_knows_when_it_is_an_atom_feed() {
        let f = Feed::new("http://google.com", "whatever", "application/atom+xml");
        assert!(!f.is_rss());
        assert!(f.is_atom());
    }
}
