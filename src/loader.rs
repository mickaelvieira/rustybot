use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

use std::string::String;
use std::fs::File;

fn load_file_content(file_path: &str) -> File {
    let path = Path::new(file_path);
    File::open(path).unwrap()
}

fn read_file_content(mut file: File) -> String {
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(size) => size,
        Err(err) => panic!("couldn't read file: {}", err.description()),
    };
    buffer
}

pub fn get_html_content() -> String {
    let file = load_file_content("/home/mickael/dev/rusty-bot/test.html");
    read_file_content(file)
}
