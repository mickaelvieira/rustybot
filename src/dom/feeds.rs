use dom::url::ParsedUrl;

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
    title: Option<String>,
    feed_type: String,
}

#[allow(dead_code)]
impl Feed {
    pub fn new<S>(url: ParsedUrl, title: Option<String>, feed_type: S) -> Feed
        where S: Into<String>
    {
        Feed {
            href: url,
            title: title,
            feed_type: feed_type.into(),
        }
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    pub fn get_title(self) -> String {
        let title = self.title.unwrap();
        title.to_owned()
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
                          Some("whatever".to_string()),
                          "application/atom+xml");
    }

    #[test]
    fn it_knows_when_it_is_a_rss_feed() {
        let f = Feed::new(ParsedUrl::new("http://google.com"),
                          Some("whatever".to_string()),
                          "application/rss+xml");
        assert!(f.is_rss());
        assert!(!f.is_atom());
    }

    #[test]
    fn it_knows_when_it_is_an_atom_feed() {
        let f = Feed::new(ParsedUrl::new("http://google.com"),
                          Some("whatever".to_string()),
                          "application/atom+xml");
        assert!(!f.is_rss());
        assert!(f.is_atom());
    }
}
