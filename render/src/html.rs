//! HTML utilities

use crate::Renderable;

/// HTML 5 doctype declaration
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
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
/// # assert_eq!(result, "<!DOCTYPE html><html><body /></html>");
/// ```
#[derive(Debug)]
pub struct HTML5Doctype;

impl Renderable for HTML5Doctype {
    fn render(self) -> String {
        "<!DOCTYPE html>".to_string()
    }
}
