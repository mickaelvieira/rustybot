use dom::parsedurl::ParsedUrl;

/// http://www.jonathanturner.org/2016/02/down-the-rabbit-hole-with-traits.html

trait FeedType {
    fn is_rss(&self) -> bool;
    fn is_atom(&self) -> bool;
}

impl FeedType for str {
    fn is_rss(&self) -> bool {
        self.to_string() == "application/atom+xml"
    }
    fn is_atom(&self) -> bool {
        self.to_string() == "application/rss+xml"
    }
}

#[allow(dead_code)]
pub struct Feed {
    href: ParsedUrl,
    title: String,
    feed_type: String,
}

#[allow(dead_code)]
impl Feed {
    fn is_rss(&self) -> bool {
        self.feed_type.as_str() == "application/rss+xml"
    }
    fn is_atom(&self) -> bool {
        self.feed_type.as_str() == "application/atom+xml"
    }
}

pub fn get_feed() -> Feed {
    Feed {
        href: ParsedUrl::from("http://google.com"),
        title: "whatever".to_string(),
        feed_type: "application/atom+xml".to_string(),
    }
}



// #[allow(dead_code)]
// enum Feeds {
//     Atom { href: ParsedUrl, title: String },
//     Rss { href: ParsedUrl, title: String },
// }

// struct Atom {
//     href: ParsedUrl,
//     title: String,
// }
//
// struct Rss {
//     href: ParsedUrl,
//     title: String,
// }
//
// #[allow(dead_code)]
// fn make_feed(type_feed: &str, href: &str, title: &str) -> Option<Feeds> {
//     if type_feed.is_atom() {
//         return Some(Feeds::Atom {
//             href: ParsedUrl::from(href),
//             title: title.to_string(),
//         });
//     } else if type_feed.is_rss() {
//         return Some(Feeds::Rss {
//             href: ParsedUrl::from(href),
//             title: title.to_string(),
//         });
//     }
//
//     None
// }
