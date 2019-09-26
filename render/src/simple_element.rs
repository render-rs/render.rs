use crate::Render;
use htmlescape::encode_attribute_w;
use std::collections::HashMap;
use std::io::{Result, Write};

type Attributes<'a> = Option<HashMap<&'a str, &'a str>>;

/// Simple HTML element tag
#[derive(Debug)]
pub struct SimpleElement<'a, T: Render> {
    /// the HTML tag name, like `html`, `head`, `body`, `link`...
    pub tag_name: &'a str,
    pub attributes: Attributes<'a>,
    pub contents: Option<T>,
}

fn write_attributes<'a, W: Write>(maybe_attributes: Attributes<'a>, writer: &mut W) -> Result<()> {
    match maybe_attributes {
        None => Ok(()),
        Some(mut attributes) => {
            for (key, value) in attributes.drain() {
                write!(writer, " {}=\"", key)?;
                encode_attribute_w(value, writer)?;
                write!(writer, "\"")?;
            }
            Ok(())
        }
    }
}

impl<T: Render> Render for SimpleElement<'_, T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        match self.contents {
            None => {
                write!(writer, "<{}", self.tag_name)?;
                write_attributes(self.attributes, writer)?;
                write!(writer, " />")
            }
            Some(renderable) => {
                write!(writer, "<{}", self.tag_name)?;
                write_attributes(self.attributes, writer)?;
                write!(writer, ">")?;
                renderable.render_into(writer)?;
                write!(writer, "</{}>", self.tag_name)
            }
        }
    }
}
