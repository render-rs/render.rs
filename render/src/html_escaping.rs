use std::fmt::{Result, Write};

pub fn escape_html<W: Write>(html: &str, writer: &mut W) -> Result {
    for c in html.chars() {
        match c {
            '>' => write!(writer, "&gt;")?,
            '<' => write!(writer, "&lt;")?,
            '"' => write!(writer, "&quot;")?,
            '&' => write!(writer, "&amp;")?,
            '\'' => write!(writer, "&apos;")?,
            c => writer.write_char(c)?,
        };
    }

    Ok(())
}
