/// Remove unwanted character from text
fn clean_html_text(text: &str) -> String {
    String::from(text).trim().to_string()
}

/// Represent some texts extracted from the HTML document
pub struct Text {
    pub value: String,
}

impl<'a> From<&'a str> for Text {
    fn from(text: &str) -> Text {
        Text { value: clean_html_text(text) }
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        self.value.to_owned()
    }
}
