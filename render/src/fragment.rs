use crate::Renderable;

/// A top-level root component to combine a same-level components
/// in a RSX fashion
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use pretty_assertions::assert_eq;
/// # use render::html::HTML5Doctype;
/// # use render_macros::html;
/// # use render::fragment::Fragment;
/// let result = html! {
///     <Fragment>
///         <a />
///         <b />
///     </Fragment>
/// };
/// assert_eq!(result, "<a /><b />");
/// ```
#[derive(Debug)]
pub struct Fragment<T: Renderable> {
    pub children: T,
}

impl<T: Renderable> Renderable for Fragment<T> {
    fn render(self) -> String {
        self.children.render()
    }
}
