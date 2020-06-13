use std::io::{Result, Write};

// Render the component to bytes.
pub fn render_to_bytes<R: Render>(el: R) -> Vec<u8> {
    let mut buf = Vec::new();
    el.render_into(&mut buf)
        .expect("no write errors when writing to a buffer");
    buf
}

/// Render the component to string. You should prefer using render_into directly.
/// Failing that, second preference is render_to_bytes. Finally remember HTML
/// isn't required to be UTF8, but a rust String is. This function will panic if
/// the HTML isn't valid UTF8.
pub fn render_to_string<R: Render>(el: R) -> String {
    String::from_utf8(render_to_bytes(el)).expect("HTML to be valid utf8")
}

/// Render a component
///
/// This is the underlying mechanism of the `#[component]` macro
pub trait Render {
    /// Render the component to a writer.
    /// Make sure you escape html correctly using the `render::html_escaping` module
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()>;
}

/// Does nothing
impl Render for () {
    fn render_into<W: Write>(self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
}

/// Renders `A`, then `B`
impl<A: Render, B: Render> Render for (A, B) {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        self.0.render_into(writer)?;
        self.1.render_into(writer)
    }
}

/// Renders `A`, then `B`, then `C`
impl<A: Render, B: Render, C: Render> Render for (A, B, C) {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        self.0.render_into(writer)?;
        self.1.render_into(writer)?;
        self.2.render_into(writer)
    }
}

/// Renders `T` or nothing
impl<T: Render> Render for Option<T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        match self {
            None => Ok(()),
            Some(x) => x.render_into(writer),
        }
    }
}

/// Renders `O` or `E`
impl<O: Render, E: Render> Render for std::result::Result<O, E> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        match self {
            Ok(o) => o.render_into(writer),
            Err(e) => e.render_into(writer),
        }
    }
}
