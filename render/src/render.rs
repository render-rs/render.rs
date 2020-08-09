use std::fmt::{Result, Write};

/// Render a component
///
/// This is the underlying mechanism of the `#[component]` macro
pub trait Render: Sized {
    /// Render the component to a writer.
    /// Make sure you escape html correctly using the `render::html_escaping` module
    fn render_into<W: Write>(self, writer: &mut W) -> Result;

    /// Render the component to string
    fn render(self) -> String {
        let mut buf = String::new();
        self.render_into(&mut buf).unwrap();
        buf
    }
}

/// Does nothing
impl Render for () {
    fn render_into<W: Write>(self, _writer: &mut W) -> Result {
        Ok(())
    }
}

/// Renders `A`, then `B`
impl<A: Render, B: Render> Render for (A, B) {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        self.0.render_into(writer)?;
        self.1.render_into(writer)
    }
}

/// Renders `A`, then `B`, then `C`
impl<A: Render, B: Render, C: Render> Render for (A, B, C) {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        self.0.render_into(writer)?;
        self.1.render_into(writer)?;
        self.2.render_into(writer)
    }
}

/// Renders `T` or nothing
impl<T: Render> Render for Option<T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        match self {
            None => Ok(()),
            Some(x) => x.render_into(writer),
        }
    }
}

impl<T: Render> Render for Vec<T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        for elem in self {
            elem.render_into(writer)?;
        }
        Ok(())
    }
}

/// Renders `O` or `E`
impl<O: Render, E: Render> Render for std::result::Result<O, E> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        match self {
            Ok(o) => o.render_into(writer),
            Err(e) => e.render_into(writer),
        }
    }
}
