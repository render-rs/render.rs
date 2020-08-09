//! HTML utilities

use crate::Render;
use std::fmt::{Result, Write};

/// HTML 5 doctype declaration
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use render::html::HTML5Doctype;
/// # use render::html;
/// # let result =
/// html! {
///     <>
///         <HTML5Doctype />
///         <html>
///             <body />
///         </html>
///     </>
/// };
/// # assert_eq!(result, "<!DOCTYPE html><html><body/></html>");
/// ```
#[derive(Debug)]
pub struct HTML5Doctype;

impl Render for HTML5Doctype {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        write!(writer, "<!DOCTYPE html>")
    }
}
