#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod child;
mod children;
mod element;
mod element_attribute;
mod element_attributes;
mod tags;

use element::Element;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Render a component tree to an HTML string, using XML-like tags.
///
/// The ergonomics are based on JSX:
///
/// ### Simple HTML elements start with a lowercase
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use pretty_assertions::assert_eq;
/// # use render_macros::html;
/// let rendered = html! { <div id={"main"}>{"Hello"}</div> };
/// assert_eq!(rendered, r#"<div id="main">Hello</div>"#);
/// ```
///
/// ### Custom components start with an uppercase
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use pretty_assertions::assert_eq;
/// # use render_macros::html;
/// use render::Renderable;
///
/// #[derive(Debug)]
/// struct Heading<'t> { title: &'t str }
///
/// impl<'t> Renderable for Heading<'t> {
///     fn render(self) -> String {
///         html! { <h1>{self.title}</h1> }
///     }
/// }
///
/// let rendered = html! { <Heading title={"Hello  world!"} /> };
///
/// assert_eq!(rendered, r#"<h1>Hello  world!</h1>"#);
/// ```
///
/// ### Punning is supported
/// but instead of expanding to `value={true}`, it expands to
/// `value={value}` like Rust's punning
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use render_macros::html;
/// # use pretty_assertions::assert_eq;
/// let class = "some_class";
///
/// let rendered = html! {
///     <div class />
/// };
///
/// assert_eq!(rendered, r#"<div class="some_class" />"#);
/// ```
///
/// ### Values are always surrounded by curly braces
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use render_macros::html;
/// # use pretty_assertions::assert_eq;
/// let rendered = html! {
///     <div id={"main"} />
/// };
///
/// assert_eq!(rendered, r#"<div id="main" />"#);
/// ```
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let el = proc_macro2::TokenStream::from(rsx(input));
    let result = quote! { ::render::Renderable::render(#el) };
    TokenStream::from(result)
}

/// Generate a renderable component tree, before rendering it
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as Element);
    let result = quote! { #el };
    TokenStream::from(result)
}
