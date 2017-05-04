use kuchiki::NodeRef;
use kuchiki::NodeDataRef;
use kuchiki::ElementData;

use dom::feeds::Feed;
use dom::feeds::FeedType;
use dom::text::HtmlString;
use dom::url::ParsedUrl;
use dom::document::Document;
use dom::document::Head;
use dom::document::Body;
use dom::document::Social;

trait DOMElement {
    fn has_attribute(&self, name: &str) -> bool;
    fn get_attribute(&self, name: &str) -> Option<String>;
}

impl DOMElement for NodeDataRef<ElementData> {
    fn has_attribute(&self, name: &str) -> bool {
        let attrs = self.attributes.borrow();
        attrs.contains(name)
    }

    fn get_attribute(&self, name: &str) -> Option<String> {
        let attrs = self.attributes.borrow();
        let value = attrs.get(name);
        if value.is_some() {
            return Some(value.unwrap().to_string());
        }
        None
    }
}

pub fn extract_data(document: &NodeRef) -> Document {
    let metas = get_meta_tags(&document);
    let links = get_link_tags(&document);
    let title = get_page_title(&document);
    let feeds = get_feeds(&links);
    let curl  = get_canonical_url(&links);
    let desc  = get_page_description(&metas);

    let fbk = get_facebook_data(&metas);
    let twt = get_twitter_data(&metas);

    let head = Head {
        title: title,
        charset: String::new(),
        feeds: feeds,
        twitter: twt,
        facebook: fbk,
        language: String::new(),
        description: desc,
        canonical_url: curl,
    };

    let body = Body {
        content: String::new(),
        headings: Vec::new(),
        links: Vec::new(),
    };

    Document {
        head: head,
        body: body,
    }
}

fn get_feeds(links: &Vec<NodeDataRef<ElementData>>) -> Vec<Feed> {
    let mut feeds: Vec<Feed> = Vec::new();
    for link in links {
        let feed_type = link.get_attribute("type");
        let feed_url = link.get_attribute("href");
        let feed_title = link.get_attribute("title");

        if feed_type.is_some() && feed_url.is_some() {
            let feed_type = feed_type.unwrap();
            let feed_url = ParsedUrl::parse(feed_url.unwrap());

            if feed_url.is_ok() && (feed_type.is_rss() || feed_type.is_atom()) {
                feeds.push(Feed::new(feed_url.unwrap(), feed_title, feed_type));
            }
        }
    }

    feeds
}

fn find_twitter_value(name: &str, nodes: &Vec<NodeDataRef<ElementData>>) -> Option<String> {
    let item = nodes.iter().find(|node| {
        let prop = node.get_attribute("property");
        prop.is_some() && prop.unwrap() == name
    });

    if item.is_some() {
        return item.unwrap().get_attribute("content");
    }

    None
}

pub fn get_twitter_data(metas: &Vec<NodeDataRef<ElementData>>) -> Option<Social> {
    let title = find_twitter_value("twitter:title", &metas);
    let url = find_twitter_value("twitter:url", &metas);
    let img = find_twitter_value("twitter:image", &metas);
    let desc = find_twitter_value("twitter:description", &metas);

    if title.is_some() && img.is_some() && desc.is_some() && url.is_some() {
        let parsed = ParsedUrl::parse(url.unwrap());
        if parsed.is_ok() {
            return Some(Social::new(title.unwrap(), desc.unwrap(), img.unwrap(), parsed.unwrap()));
        }
    }

    None
}

fn find_facebook_value<'a>(name: &str,
                           nodes: &'a Vec<NodeDataRef<ElementData>>)
                           -> Option<String> {

    let item = nodes.iter().find(|node| {
        let prop = node.get_attribute("property");
        prop.is_some() && prop.unwrap() == name
    });

    if item.is_some() {
        return item.unwrap().get_attribute("content");
    }

    None
}

pub fn get_facebook_data(metas: &Vec<NodeDataRef<ElementData>>) -> Option<Social> {
    let title = find_facebook_value("og:title", &metas);
    let url = find_facebook_value("og:url", &metas);
    let img = find_facebook_value("og:image", &metas);
    let desc = find_facebook_value("og:description", &metas);

    if title.is_some() && img.is_some() && desc.is_some() && url.is_some() {
        let parsed = ParsedUrl::parse(url.unwrap());
        if parsed.is_ok() {
            return Some(Social::new(title.unwrap(), desc.unwrap(), img.unwrap(), parsed.unwrap()));
        }
    }

    None
}

fn get_page_title(document: &NodeRef) -> Option<String> {
    let collection = document.select("head title");
    if collection.is_ok() {
        let mut tags = collection.unwrap()
            .collect::<Vec<NodeDataRef<ElementData>>>();

        if tags.len() > 0 {
            return Some(String::from_html(tags.remove(0).text_contents()));
        }
    }

    return None;
}

fn get_page_description(metas: &Vec<NodeDataRef<ElementData>>) -> Option<String> {
    let item = metas.iter().find(|node| {
        let prop = node.get_attribute("name");
        prop.is_some() && prop.unwrap() == "description"
    });

    if item.is_some() {
        return item.unwrap().get_attribute("content");
    }

    None
}

fn get_canonical_url(links: &Vec<NodeDataRef<ElementData>>) -> Option<ParsedUrl> {
    for link in links {
        let rel_attr = link.get_attribute("rel");
        let feed_url = link.get_attribute("href");
        if feed_url.is_some() && rel_attr.is_some() && rel_attr.unwrap() == "canonical" {
            let canonical_url = ParsedUrl::parse(feed_url.unwrap());

            if canonical_url.is_ok() {
                return Some(canonical_url.unwrap());
            }
        }
    }

    None
}

pub fn get_meta_tags(document: &NodeRef) -> Vec<NodeDataRef<ElementData>> {
    let collection = document.select("head meta");
    if collection.is_ok() {
        return collection.unwrap()
            .collect::<Vec<NodeDataRef<ElementData>>>();
    }

    Vec::new()
}

fn get_link_tags(document: &NodeRef) -> Vec<NodeDataRef<ElementData>> {
    let collection = document.select("head link");
    if collection.is_ok() {
        return collection.unwrap()
            .collect::<Vec<NodeDataRef<ElementData>>>();
    }

    Vec::new()
}

#[cfg(test)]
mod tests {

    extern crate kuchiki;

    use super::*;
    use kuchiki::traits::*;

    #[test]
    fn it_returns_none_when_there_is_no_title() {
        let html = "
            <html>
            <head></head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let title = get_page_title(&document);
        assert!(title.is_none());
    }

    #[test]
    fn it_returns_the_title() {
        let html = "
            <html>
            <head>
                <title>The Title</title>
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let title = get_page_title(&document);
        assert_eq!(title, Some("The Title".to_string()));
    }

    #[test]
    fn it_returns_an_empty_vector_when_there_is_no_meta_tags() {
        let html = "
            <html>
            <head></head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let metas = get_meta_tags(&document);
        assert_eq!(metas.len(), 0);
    }

    #[test]
    fn it_collects_in_a_vector_the_meta_tags() {
        let html = "
            <html>
            <head>
                <meta name=\"title\"  content=\"The title\" />
                <meta name=\"description\"  content=\"The description\" />
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let metas = get_meta_tags(&document);
        assert_eq!(metas.len(), 2);
    }

    #[test]
    fn it_returns_an_empty_vector_when_there_is_no_link_tags() {
        let html = "
            <html>
            <head></head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let links = get_link_tags(&document);
        assert_eq!(links.len(), 0);
    }

    #[test]
    fn it_collects_the_link_tags() {
        let html = "
            <html>
            <head>
                <link rel=\"apple-touch-icon-precomposed\" sizes=\"144×144\" />
                <link rel=\"apple-touch-icon-precomposed\" sizes=\"114×114\" />
                <link rel=\"apple-touch-icon-precomposed\" />
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let links = get_link_tags(&document);
        assert_eq!(links.len(), 3);
    }

    #[test]
    fn it_returns_an_empty_vector_when_there_is_no_feeds() {
        let html = "
            <html>
            <head>
                <link rel=\"apple-touch-icon-precomposed\" sizes=\"144×144\" />
                <link rel=\"apple-touch-icon-precomposed\" sizes=\"114×114\" />
                <link rel=\"apple-touch-icon-precomposed\" />
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let links = get_link_tags(&document);
        let feeds = get_feeds(&links);
        assert_eq!(links.len(), 3);
        assert_eq!(feeds.len(), 0);
    }

    #[test]
    fn it_collects_the_feed_tags() {
        let html = "
            <html>
            <head>
                <link rel=\"apple-touch-icon-precomposed\" sizes=\"144×144\" />
                <link rel=\"apple-touch-icon-precomposed\" sizes=\"114×114\" />
                <link rel=\"apple-touch-icon-precomposed\" />
                <link href=\"http://cargo.io/rss.rs\" type=\"application/rss+xml\" />
                <link href=\"http://cargo.io/atom.rs\" type=\"application/atom+xml\" />
                <link href=\"cargo.io/atom.rs\" type=\"application/atom+xml\" />
                <link href=\"http://cargo.io/atom.rs\" type=\"application/xml\" />
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let links = get_link_tags(&document);
        let feeds = get_feeds(&links);
        assert_eq!(links.len(), 7);
        assert_eq!(feeds.len(), 2);
    }

    #[test]
    fn it_returns_none_when_there_is_no_description() {
        let html = "
            <html>
            <head>
                <meta name=\"keywords\" content=\"meta description search results.\">
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let metas = get_meta_tags(&document);
        let description = get_page_description(&metas);
        assert!(description.is_none());
    }

    #[test]
    fn it_returns_the_description() {
        let html = "
            <html>
            <head>
                <link rel=\"canonical\" href=\"http://example.com/wordpress/seo-plugin/\">
                <meta name=\"description\" content=\"This is an example of a meta description\">
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let metas = get_meta_tags(&document);
        let description = get_page_description(&metas);
        assert!(description.is_some());
        assert_eq!(description.unwrap(), "This is an example of a meta description".to_string());
    }

    #[test]
    fn it_returns_none_when_there_is_no_canonical_url() {
        let html = "
            <html>
            <head>
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let links = get_link_tags(&document);
        let url = get_canonical_url(&links);
        assert!(url.is_none());
    }

    #[test]
    fn it_retrieves_the_canonical_url() {
        let html = "
            <html>
            <head>
                <link rel=\"canonical\" href=\"http://example.com/wordpress/seo-plugin/\">
            </head>
            <body></body>"
            .to_string();

        let document = kuchiki::parse_html().one(html);
        let links = get_link_tags(&document);
        let url = get_canonical_url(&links);
        assert!(url.is_some());
        assert_eq!(url.unwrap().to_string(), "http://example.com/wordpress/seo-plugin/".to_string());
    }
}
