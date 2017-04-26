// #[macro_use]
// extern crate string_cache;
extern crate rustybot;
extern crate kuchiki;
extern crate url;

use rustybot::loader;
use rustybot::extractor::extract_data;
use kuchiki::traits::*;

fn main() {
    let html = loader::get_html_content();
    let document = kuchiki::parse_html().one(html);

    let _doc = extract_data(&document);

    println!("{:?}", _doc);
    // let metas = get_meta_tags(&document);
    // let s = get_facebook_data(&metas);
    // let t = get_twitter_data(&metas);
    //
    // println!("{:?}", s.unwrap());
    // println!("{:?}", t.unwrap());

    // let metas = get_meta_tags(&document);
    // println!("-> {:?} ", metas.len());
    //
    // let links = get_link_tags(&document);
    // println!("-> {:?} ", links.len());
    //
    // let title = get_page_title(&document);
    // println!("-> {:?} ", title.unwrap());
    //
    // let feeds = get_feeds(&links);
    // println!("-> {:?} ", feeds.len());

    // let css = "head";
    // let mut coll = document.select(css).unwrap();
    // let first = coll.nth(0).unwrap();
    // let node = first.as_node();
    // let mut meta = node.select("meta").unwrap();

    // let m = meta.nth(0).unwrap();
    // let mut n = m.as_node();
    // let attrs = m.attributes.borrow();
    // let mut attrs = e.attributes;
    // let a = attrs.borrow();

    // println!("Contains {:?} ", e.attributes.into_inner().contains("name"));
    // println!("-> {:?} ", m.as_node().to_string());
    // println!("-> {:?} ", attrs.contains("name"));

    // let node = get_node(&document);
    // println!("{}", node.unwrap().to_string());

    // let coll = document.select("head");
    // if coll.is_ok() {
    //     let mut iter = coll.unwrap();
    //     if let Some(head) = iter.next() {
    //         println!("{}", head.as_node());
    //     }
    // }
    //
    // let head = match document.select("head") {
    //     Ok(r) => r,
    //     Err(e) => panic!("aaaaaaaaaaaaaaah!"),
    // };


}
