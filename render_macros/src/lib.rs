extern crate proc_macro;

mod child;
mod children;
mod element;
mod element_attribute;
mod element_attributes;
mod function_component;
mod tags;

use element::Element;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

/// Render a component tree to an HTML string, using XML-like tags.
///
/// The ergonomics are based on JSX:
///
/// ### Simple HTML elements start with a lowercase
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use render_macros::html;
/// let rendered = html! { <div id={"main"}>{"Hello"}</div> };
/// assert_eq!(rendered, r#"<div id="main">Hello</div>"#);
/// ```
///
/// ### Custom components start with an uppercase
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use render_macros::{html, rsx};
/// use render::Render;
///
/// #[derive(Debug)]
/// struct Heading<'t> { title: &'t str }
///
/// impl<'t> Render for Heading<'t> {
///     fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
///         Render::render_into(rsx! { <h1>{self.title}</h1> }, writer)
///     }
/// }
///
/// let rendered = html! { <Heading title={"Hello world!"} /> };
///
/// assert_eq!(rendered, r#"<h1>Hello world!</h1>"#);
/// ```
///
/// ### Values are always surrounded by curly braces
///
/// ```rust
/// # use render_macros::html;
/// # use pretty_assertions::assert_eq;
/// let rendered = html! {
///     <div id={"main"} />
/// };
///
/// assert_eq!(rendered, r#"<div id="main"/>"#);
/// ```
///
/// ### HTML entities can accept dashed-separated value
///
/// ```rust
/// # use render_macros::html;
/// # use pretty_assertions::assert_eq;
/// let rendered = html! {
///     <div data-testid={"sometestid"} />
/// };
///
/// assert_eq!(rendered, r#"<div data-testid="sometestid"/>"#);
/// ```
///
/// ### Custom components can't accept dashed-separated values
///
/// ```compile_fail
/// # use render_macros::html;
/// // This will fail the compilation:
/// let rendered = html! {
///     <MyElement data-testid={"some test id"} />
/// };
/// ```
///
/// ### Punning is supported
/// but instead of expanding to `value={true}`, it expands to
/// `value={value}` like Rust's punning
///
/// ```rust
/// # use render_macros::html;
/// # use pretty_assertions::assert_eq;
/// let class = "someclass";
///
/// let rendered = html! {
///     <div class />
/// };
///
/// assert_eq!(rendered, r#"<div class="someclass"/>"#);
/// ```
///
/// ### Punning is not supported for dashed-delimited attributes
///
/// ```compile_fail
/// # use render_macros::html;
///
/// let rendered = html! {
///     <div this-wont-work />
/// };
///
/// assert_eq!(rendered, r#"<div class="some_class"/>"#);
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    let el = proc_macro2::TokenStream::from(rsx(input));
    let result = quote! { ::render::Render::render(#el) };
    TokenStream::from(result)
}

/// Generate a renderable component tree, before rendering it
#[proc_macro]
#[proc_macro_error]
pub fn rsx(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as Element);
    let result = quote! { #el };
    TokenStream::from(result)
}

/// A syntactic sugar for implementing [`Render`](../render/trait.Render.html) conveniently
/// using functions.
///
/// This attribute should be above a stand-alone function definition that returns a
/// [`String`](std::string::String):
///
/// ```rust
/// # use render_macros::{component, rsx};
/// #
/// #[component]
/// fn UserFn(name: String) {
///     rsx! { <div>{format!("Hello, {}", name)}</div> }
/// }
/// ```
///
/// Practically, this is exactly the same as using the [Render](../render/trait.Render.html) trait:
///
/// ```rust
/// # use render_macros::{component, rsx, html};
/// # use render::Render;
/// # use pretty_assertions::assert_eq;
/// #
/// #[derive(Debug)]
/// struct User { name: String }
///
/// impl render::Render for User {
///     fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
///         Render::render_into(rsx! { <div>{format!("Hello, {}", self.name)}</div> }, writer)
///     }
/// }
///
/// # #[component]
/// # fn UserFn(name: String) {
/// #     rsx! { <div>{format!("Hello, {}", name)}</div> }
/// # }
/// #
/// # let from_fn = html! {
/// #     <UserFn name={String::from("Schniz")} />
/// # };
/// #
/// # let from_struct = html! {
/// #     <User name={String::from("Schniz")} />
/// # };
/// #
/// # assert_eq!(from_fn, from_struct);
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);
    function_component::create_function_component(f)
}
