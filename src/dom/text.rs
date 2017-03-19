use std::string::ToString;

/// Remove unwanted character from text
fn clean_html_text(text: &str) -> String {
    let text = text.to_string();
    let words: Vec<&str> = text.split_whitespace().collect();
    words.join(" ").trim().to_string()
}

/// Represent some texts extracted from the HTML document
#[derive(Debug)]
pub struct Text {
    pub value: String,
}

impl Text {
    pub fn new(text: &str) -> Text {
        Text { value: clean_html_text(text) }
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        self.value.to_owned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let _ = Text::new("Rust is awesome");
    }

    #[test]
    fn it_removes_leading_spaces() {
        let s = Text::new("   Rust is awesome");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }

    #[test]
    fn it_removes_trailing_spaces() {
        let s = Text::new("Rust is awesome   ");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }

    #[test]
    fn it_removes_extra_spaces() {
        let s = Text::new("Rust is        awesome   ");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }

    #[test]
    fn it_removes_extra_new_line_characters() {
        let s = Text::new("
        Rust
    is
    awesome
");
        assert_eq!(s.to_string(), "Rust is awesome".to_string());
    }
}
