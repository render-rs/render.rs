use crate::Renderable;
use std::collections::HashMap;

/// Simple HTML element tag
#[derive(Debug)]
pub struct SimpleElement<'a, T: Renderable> {
    /// the HTML tag name, like `html`, `head`, `body`, `link`...
    pub tag_name: &'a str,
    pub attributes: Option<HashMap<&'a str, &'a str>>,
    pub contents: Option<T>,
}

fn attributes_to_string<Key: std::fmt::Display + std::hash::Hash, Value: std::fmt::Debug>(
    opt: &Option<HashMap<Key, Value>>,
) -> String {
    match opt {
        None => "".to_string(),
        Some(map) => {
            let s: String = map
                .iter()
                .map(|(key, value)| format!(" {}={:?}", key, value))
                .collect();
            s
        }
    }
}

impl<'a, T: Renderable> Renderable for SimpleElement<'a, T> {
    fn render(self) -> String {
        let attrs = attributes_to_string(&self.attributes);
        match self.contents {
            None => format!("<{}{} />", self.tag_name, attrs),
            Some(renderable) => format!(
                "<{tag_name}{attrs}>{contents}</{tag_name}>",
                tag_name = self.tag_name,
                attrs = attrs,
                contents = renderable.render()
            ),
        }
    }
}
