use crate::html_escaping::escape_html;
use crate::Render;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Result, Write};

type AV<'a> = Option<Cow<'a, str>>;

pub trait ToAttribute<'a> {
    fn from_value(self) -> AV<'a>;
}

impl<'a> ToAttribute<'a> for Option<Cow<'a, str>> {
    fn from_value(self) -> AV<'a> {
        self
    }
}

impl<'a> ToAttribute<'a> for () {
    fn from_value(self) -> AV<'a> {
        None
    }
}

impl<'a> ToAttribute<'a> for String {
    fn from_value(self) -> AV<'a> {
        Some(Cow::Owned(self))
    }
}

impl<'a> ToAttribute<'a> for &'a str {
    fn from_value(self) -> AV<'a> {
        Some(Cow::Borrowed(self))
    }
}

impl<'a> ToAttribute<'a> for Option<&'a str> {
    fn from_value(self) -> AV<'a> {
        self.map(|v| Cow::Borrowed(v))
    }
}

impl<'a> ToAttribute<'a> for Option<String> {
    fn from_value(self) -> AV<'a> {
        self.map(|v| Cow::Owned(v))
    }
}

macro_rules! impl_primitive {
    [$($num: ty),+] => {
        $(
            impl<'a> ToAttribute<'a> for $num {
                fn from_value(self) -> AV<'a> {
                    Some(Cow::Owned(self.to_string()))
                }
            }
        )+
    };
}
impl_primitive![u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool];

impl<'a> ToAttribute<'a> for Cow<'a, str> {
    fn from_value(self) -> AV<'a> {
        Some(self)
    }
}

type Attributes<'a> = Option<HashMap<&'a str, AV<'a>>>;

/// Simple HTML element tag
#[derive(Debug)]
pub struct SimpleElement<'a, T: Render> {
    /// the HTML tag name, like `html`, `head`, `body`, `link`...
    pub tag_name: &'a str,
    pub attributes: Attributes<'a>,
    pub contents: Option<T>,
}

fn write_attributes<'a, W: Write>(maybe_attributes: Attributes<'a>, writer: &mut W) -> Result {
    match maybe_attributes {
        None => Ok(()),
        Some(mut attributes) => {
            for (key, maybe_value) in attributes.drain() {
                if let Some(value) = maybe_value {
                    write!(writer, " {}=\"", key)?;
                    escape_html(&value, writer)?;
                    write!(writer, "\"")?;
                }
            }
            Ok(())
        }
    }
}

impl<T: Render> Render for SimpleElement<'_, T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        match self.contents {
            None => {
                write!(writer, "<{}", self.tag_name)?;
                write_attributes(self.attributes, writer)?;
                write!(writer, "/>")
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
