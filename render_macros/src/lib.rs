#![feature(proc_macro_diagnostic)]

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
///
/// ### HTML entities can accept dashed-separated value
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use render_macros::html;
/// # use pretty_assertions::assert_eq;
/// let rendered = html! {
///     <div data-testid={"some test id"} />
/// };
///
/// assert_eq!(rendered, r#"<div data-testid="some test id" />"#);
/// ```
///
/// ### Custom components can't accept dashed-separated values
///
/// ```compile_fail
/// # #![feature(proc_macro_hygiene)]
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
/// ### Punning is not supported for dashed-delimited attributes
///
/// ```compile_fail
/// # #![feature(proc_macro_hygiene)]
/// # use render_macros::html;
///
/// let rendered = html! {
///     <div this-wont-work />
/// };
///
/// assert_eq!(rendered, r#"<div class="some_class" />"#);
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

/// A syntactic sugar for implementing [`Renderable`](../render/trait.Renderable.html) conveniently
/// using functions.
///
/// This attribute should be above a stand-alone function definition that returns a
/// [`String`](std::string::String):
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use render_macros::{component, html};
/// #
/// #[component]
/// fn UserFn(name: String) -> String {
///     html! { <div>{format!("Hello, {}", name)}</div> }
/// }
/// ```
///
/// Practically, this is exactly the same as using the [Renderable](../render/trait.Renderable.html) trait:
///
/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use render_macros::{component, html};
/// # use render::Renderable;
/// # use pretty_assertions::assert_eq;
/// #
/// #[derive(Debug)]
/// struct User { name: String }
///
/// impl render::Renderable for User {
///     fn render(self) -> String {
///         html! { <div>{format!("Hello, {}", self.name)}</div> }
///     }
/// }
///
/// # #[component]
/// # fn UserFn(name: String) -> String {
/// #     html! { <div>{format!("Hello, {}", name)}</div> }
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
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);
    function_component::to_component(f)
}
