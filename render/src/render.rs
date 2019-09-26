use std::io::{Result, Write};

pub trait Render: Sized {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()>;

    fn render(self) -> String {
        let mut buf = Vec::new();
        self.render_into(&mut buf).unwrap();
        std::str::from_utf8(&buf).unwrap().to_string()
    }
}

impl Render for () {
    fn render_into<W: Write>(self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
}

impl<A: Render, B: Render> Render for (A, B) {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        self.0.render_into(writer)?;
        self.1.render_into(writer)
    }
}

impl<A: Render, B: Render, C: Render> Render for (A, B, C) {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        self.0.render_into(writer)?;
        self.1.render_into(writer)?;
        self.2.render_into(writer)
    }
}

impl<T: Render> Render for Option<T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result<()> {
        match self {
            None => Ok(()),
            Some(x) => x.render_into(writer),
        }
    }
}
