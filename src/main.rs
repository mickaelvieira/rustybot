
extern crate rustybot;
extern crate kuchiki;
extern crate url;

use rustybot::loader;
use kuchiki::traits::*;

fn main() {
    println!("Hello, world!");

    let html = loader::get_html_content();
    let document = kuchiki::parse_html().one(html);

    println!("Now {:?} will print!", document);
}
