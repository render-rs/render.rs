use crate::Renderable;
use htmlescape::encode_minimal;

/// Renders an escaped-html string
impl Renderable for String {
    fn render(self) -> String {
        encode_minimal(&self)
    }
}

/// Renders an escaped-html string
impl Renderable for &str {
    fn render(self) -> String {
        encode_minimal(self)
    }
}

/// A raw (unencoded) html string
#[derive(Debug)]
pub struct Raw<'s>(&'s str);

impl<'s> From<&'s str> for Raw<'s> {
    fn from(s: &'s str) -> Self {
        Raw(s)
    }
}

/// A raw (unencoded) html string
impl<'s> Renderable for Raw<'s> {
    fn render(self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_html() {
        let rendered = "<Hello />".render();
        assert_eq!(rendered, "&lt;Hello /&gt;");
    }

    #[test]
    fn allows_raw_text() {
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
