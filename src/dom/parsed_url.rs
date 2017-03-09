use url::Url;
use regex::Regex;

pub struct ParsedUrl {
    parsed: Url,
}

impl ParsedUrl {
    pub fn new(url: &str) -> ParsedUrl {
        ParsedUrl { parsed: Url::parse(url).unwrap() }
    }

    pub fn keywords(&self) -> Vec<String> {
        let regex = Regex::new(r"\.([a-z]+)$").unwrap();
        let path = regex.replace(self.parsed.path(), "");

        path.replace("-", " ")
            .replace("_", " ")
            .replace(".", " ")
            .replace("/", " ")
            .replace("%E2%80%94", " ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}

impl ToString for ParsedUrl {
    fn to_string(&self) -> String {
        self.parsed.as_str().to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let p = ParsedUrl::new("http://crates.io");
        assert_eq!(p.to_string(), "http://crates.io/");
    }

    #[test]
    fn it_handles_non_standards_hyphens() {
        let u = "http://www.example.com/state—regional-govt—politics/";
        let p = ParsedUrl::new(u);
        let e = "http://www.example.com/state%E2%80%94regional-govt%E2%80%94politics/";
        assert_eq!(p.to_string(), e);
    }

    #[test]
    fn it_retrieves_the_keywords() {
        let u = "http://example.com/awesome_section/my—great-article.php";
        let p = ParsedUrl::new(u);

        assert_eq!(p.keywords(),
                   vec!["awesome", "section", "my", "great", "article"]);
    }

    #[test]
    #[should_panic]
    fn it_fails_when_the_url_is_not_valid() {
        let _ = ParsedUrl::new("/hello.rs");
    }
}
