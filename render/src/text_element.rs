use crate::html_escaping::escape_html;
use crate::Render;
use std::fmt::{Result, Write};

impl Render for String {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        escape_html(&self, writer)
    }
}

impl Render for &str {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        escape_html(self, writer)
    }
}

impl Render for std::borrow::Cow<'_, str> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        escape_html(&self, writer)
    }
}

/// A raw (unencoded) html string
#[derive(Debug)]
pub struct Raw(String);

impl From<&str> for Raw {
    fn from(s: &str) -> Self {
        Raw(s.to_string())
    }
}


impl From<String> for Raw {
    fn from(s: String) -> Self {
        Raw(s)
    }
}

/// A raw (unencoded) html string
impl Render for Raw {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        write!(writer, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_html() {
        use pretty_assertions::assert_eq;
        let rendered = "<Hello />".render();
        assert_eq!(rendered, "&lt;Hello /&gt;");
    }

    #[test]
    fn allows_raw_text() {
        use pretty_assertions::assert_eq;
        let rendered = Raw::from("<Hello />").render();
        assert_eq!(rendered, "<Hello />");
    }
}

/// Creates a raw (unencoded) html string
#[macro_export]
macro_rules! raw {
    ($text:expr) => {
        ::render::Raw::from($text)
    };
}
