use url::Url;
use regex::Regex;

fn clean_chars(url: &str) -> String {
    url.trim()
       .replace("—", "-")
       .to_lowercase()
}

pub struct ParsedUrl {
    original: String,
    parsed: Url,
}

impl ParsedUrl {
    pub fn new(url: &str) -> ParsedUrl {
        ParsedUrl {
            original: String::from(url),
            parsed: Url::parse(clean_chars(url).as_str()).unwrap(),
        }
    }

    pub fn keywords(&self) -> Vec<String> {
        let regex = Regex::new(r"\.([a-z]+)$").unwrap();
        let path = regex.replace(self.parsed.path(), "");

        path.replace("-", " ")
            .replace("_", " ")
            .replace(".", " ")
            .replace("/", " ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn is_secure(&self) -> bool {
        self.parsed.scheme() == "https"
    }
}

impl ToString for ParsedUrl {
    fn to_string(&self) -> String {
        self.original.to_owned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let u = "http://crates.io";
        let p = ParsedUrl::new(u);
        assert_eq!(p.to_string(), u);
    }

    #[test]
    fn it_knows_when_it_is_secure() {
        let p1 = ParsedUrl::new("http://crates.io");
        assert!(!p1.is_secure());

        let p2 = ParsedUrl::new("https://crates.io");
        assert!(p2.is_secure());
    }

    #[test]
    fn it_handles_non_standards_hyphens() {
        let u = "http://www.example.com/state—regional-govt—politics/";
        let p = ParsedUrl::new(u);
        assert_eq!(p.to_string(), u);
    }

    #[test]
    fn it_retrieves_the_keywords() {
        let u = "http://example.com/awesome_section/my—great-article.php";
        let p = ParsedUrl::new(u);

        assert_eq!(p.keywords(),
                   vec!["awesome", "section", "my", "great", "article"]);
    }

    #[test]
    fn it_handles_uppercase_urls() {
        let u = "HTTP://EXAMPLE.COM/AWESOME_SECTION/MY—GREAT-ARTICLE.PHP";
        let p = ParsedUrl::new(u);

        assert_eq!(p.to_string(), u);
        assert!(!p.is_secure());
        assert_eq!(p.keywords(),
                   vec!["awesome", "section", "my", "great", "article"]);
    }

    #[test]
    #[should_panic]
    fn it_fails_when_the_url_is_not_valid() {
        let _ = ParsedUrl::new("/hello.rs");
    }
}
