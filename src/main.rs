
extern crate rustybot;
extern crate kuchiki;

use kuchiki::traits::*;

fn main() {
    println!("Hello, world!");

    let html = rustybot::loader::get_html_content();
    let document = kuchiki::parse_html().one(html);

    println!("Now {:?} will print!", document);
}
