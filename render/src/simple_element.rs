use crate::html_escaping::escape_html;
use crate::Render;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Result, Write};

type Attributes<'a> = Option<HashMap<&'a str, Cow<'a, str>>>;

/// Simple HTML element tag
#[derive(Debug)]
pub struct SimpleElement<'a, T: Render> {
    /// the HTML tag name, like `html`, `head`, `body`, `link`...
    pub tag_name: &'a str,
    pub attributes: Attributes<'a>,
    pub contents: Option<T>,
    pub self_closing: bool,
}

fn write_attributes<'a, W: Write>(maybe_attributes: Attributes<'a>, writer: &mut W) -> Result {
    match maybe_attributes {
        None => Ok(()),
        Some(mut attributes) => {
            for (key, value) in attributes.drain() {
                write!(writer, " {}=\"", key)?;
                escape_html(&value, writer)?;
                write!(writer, "\"")?;
            }
            Ok(())
        }
    }
}

impl<T: Render> Render for SimpleElement<'_, T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        if self.self_closing {
            write!(writer, "<{}", self.tag_name)?;
            write_attributes(self.attributes, writer)?;
            write!(writer, "/>")
        } else {
            write!(writer, "<{}", self.tag_name)?;
            write_attributes(self.attributes, writer)?;
            write!(writer, ">")?;
            self.contents.render_into(writer)?;
            write!(writer, "</{}>", self.tag_name)
        }
    }
}
