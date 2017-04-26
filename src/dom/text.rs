use std::string::ToString;

pub trait HtmlString {
    fn from_html<S>(text: S) -> String where S: Into<String>;
}

impl HtmlString for String {
    fn from_html<S>(text: S) -> String where S: Into<String> {
        let text = text.into();
        let words: Vec<&str> = text.split_whitespace().collect();
        words.join(" ").trim().to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let _ = String::from_html("Rust is awesome");
    }

    #[test]
    fn it_removes_leading_spaces() {
        let s = String::from_html("   Rust is awesome");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }

    #[test]
    fn it_removes_trailing_spaces() {
        let s = String::from_html("Rust is awesome   ");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }

    #[test]
    fn it_removes_extra_spaces() {
        let s = String::from_html("Rust is        awesome   ");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }

    #[test]
    fn it_removes_extra_new_line_characters() {
        let s = String::from_html("
        Rust
    is
    awesome
");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }
}
