extern crate kuchiki;

use url::Url;
use kuchiki::NodeRef;
use kuchiki::NodeDataRef;
use kuchiki::ElementData;
// use regex::Regex;
use dom::feeds::Feed;
use dom::text::Text;
use dom::parsed_url::ParsedUrl;
use dom::document::Document;
use dom::document::Head;
use dom::document::Body;
use dom::document::Social;

pub fn extract_data(document: &NodeRef) -> Document {

    let metas = get_meta_tags(&document);
    let links = get_link_tags(&document);
    let title = get_page_title(&document);

    let feeds = get_feeds(&links);

    let fbk = get_facebook_data(&metas);
    let twt = get_twitter_data(&metas);

    let head = Head {
        title: Text::new(title.unwrap().as_str()),
        charset: String::new(),
        feeds: feeds,
        twitter: twt.unwrap(),
        facebook: fbk.unwrap(),
        language: String::new(),
        description: String::new(),
        canonical_url: ParsedUrl::new("http://google.com"),
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

#[allow(dead_code)]
fn get_feeds(links: &Vec<NodeDataRef<ElementData>>) -> Vec<Feed> {
    let mut feeds: Vec<Feed> = Vec::new();
    for link in links {
        let attrs = link.attributes.borrow();
        if attrs.contains("href") && attrs.contains("type") {
            let feed_type = attrs.get("type").unwrap();
            let feed_url = attrs.get("href").unwrap();
            let feed_title = attrs.get("title").unwrap_or("");
            if feed_type == "application/rss+xml" || feed_type == "application/atom+xml" {
                feeds.push(Feed::new(feed_url, feed_title, feed_type));
            }
        }
    }

    feeds
}

fn find_twitter_value(name: &str, nodes: &Vec<NodeDataRef<ElementData>>) -> Option<String> {
    for node in nodes {
        let attrs = node.attributes.borrow();
        let prop = attrs.get("property").unwrap_or("");
        let ctt = attrs.get("content").unwrap_or("");

        if prop == name {
            return Some(ctt.to_string());
        }
    }

    None
}

pub fn get_twitter_data(metas: &Vec<NodeDataRef<ElementData>>) -> Option<Social> {
    let title = find_twitter_value("twitter:title", &metas).unwrap_or(String::new());
    let url = find_twitter_value("twitter:url", &metas).unwrap_or(String::new());
    let img = find_twitter_value("twitter:image", &metas).unwrap_or(String::new());
    let desc = find_twitter_value("twitter:description", &metas).unwrap_or(String::new());

    let parsed = Url::parse(url.as_str());

    if parsed.is_ok() {
        return Some(Social::new(title.as_str(), desc.as_str(), img.as_str(), url.as_str()));
    }

    None
}

fn find_facebook_value(name: &str, nodes: &Vec<NodeDataRef<ElementData>>) -> Option<String> {
    for node in nodes {
        let attrs = node.attributes.borrow();
        let prop = attrs.get("property").unwrap_or("");
        let ctt = attrs.get("content").unwrap_or("");

        if prop == name {
            return Some(ctt.to_string());
        }
    }

    None
}

pub fn get_facebook_data(metas: &Vec<NodeDataRef<ElementData>>) -> Option<Social> {
    let title = find_facebook_value("og:title", &metas).unwrap_or(String::new());
    let url = find_facebook_value("og:url", &metas).unwrap_or(String::new());
    let img = find_facebook_value("og:image", &metas).unwrap_or(String::new());
    let desc = find_facebook_value("og:description", &metas).unwrap_or(String::new());

    let parsed = Url::parse(url.as_str());

    if parsed.is_ok() {
        return Some(Social::new(title.as_str(), desc.as_str(), img.as_str(), url.as_str()));
    }

    None
}

#[allow(dead_code)]
fn get_page_title(document: &NodeRef) -> Option<String> {
    let collection = document.select("head title");
    if collection.is_ok() {
        let mut tags = collection
            .unwrap()
            .collect::<Vec<NodeDataRef<ElementData>>>();

        if tags.len() > 0 {
            return Some(tags.remove(0).text_contents());
        }
    }

    return None
}

#[allow(dead_code)]
pub fn get_meta_tags(document: &NodeRef) -> Vec<NodeDataRef<ElementData>> {
    let collection = document.select("head meta");
    if collection.is_ok() {
        return collection
            .unwrap()
            .collect::<Vec<NodeDataRef<ElementData>>>();
    }

    Vec::new()
}

#[allow(dead_code)]
fn get_link_tags(document: &NodeRef) -> Vec<NodeDataRef<ElementData>> {
    let collection = document.select("head link");
    if collection.is_ok() {
        return collection
            .unwrap()
            .collect::<Vec<NodeDataRef<ElementData>>>();
    }

    Vec::new()
}

#[cfg(test)]
mod tests {

    use super::*;
    use kuchiki::traits::*;

    #[test]
    fn it_returns_none_when_there_is_no_title() {
        let html = "
            <html>
            <head></head>
            <body></body>".to_string();

        let document = kuchiki::parse_html().one(html);
        let title = get_page_title(&document);
        assert_eq!(title, None);
    }

    #[test]
    fn it_returns_the_title() {
        let html = "
            <html>
            <head>
                <title>The Title</title>
            </head>
            <body></body>".to_string();

        let document = kuchiki::parse_html().one(html);
        let title = get_page_title(&document);
        assert_eq!(title, Some("The Title".to_string()));
    }

    #[test]
    fn it_returns_an_empty_vector_when_there_is_no_meta_tags() {
        let html = "
            <html>
            <head></head>
            <body></body>".to_string();

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
            <body></body>".to_string();

        let document = kuchiki::parse_html().one(html);
        let metas = get_meta_tags(&document);
        assert_eq!(metas.len(), 2);
    }

    #[test]
    fn it_returns_an_empty_vector_when_there_is_no_link_tags() {
        let html = "
            <html>
            <head></head>
            <body></body>".to_string();

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
            <body></body>".to_string();

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
            <body></body>".to_string();

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
            </head>
            <body></body>".to_string();

        let document = kuchiki::parse_html().one(html);
        let links = get_link_tags(&document);
        let feeds = get_feeds(&links);
        assert_eq!(links.len(), 5);
        assert_eq!(feeds.len(), 2);
    }


}
