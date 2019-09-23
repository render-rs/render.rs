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

/// ```rust
/// # #![feature(proc_macro_hygiene)]
/// # use render_macros::{component, html};
/// # use pretty_assertions::assert_eq;
///
/// #[component]
/// fn User(name: String) -> String {
///     html! { <div>{format!("Hello, {}", name)}</div> }
/// }
///
/// /// #[derive(Debug)]
/// /// struct User { name: String };
/// ///
/// /// impl Renderable for User {
/// ///     fn render(self) -> String {
/// ///         let User { name } = self;
/// ///         html! { <div>{format!("Hello, {}", name)}</div> }
/// ///     }
/// /// }
///
/// let rendered = html! {
///     <User name={String::from("Schniz")} />
/// };
///
/// assert_eq!(rendered, r#"<div>Hello, Schniz</div>"#);
/// ```
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let syn::ItemFn {
        attrs: _attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as syn::ItemFn);

    let struct_name = sig.ident;
    let (impl_generics, ty_generics, where_clause) = sig.generics.split_for_impl();
    let inputs = sig.inputs;

    let inputs_block = if inputs.len() > 0 {
        quote!({ #inputs })
    } else {
        quote!(;)
    };

    let inputs_reading = if inputs.len() == 0 {
        quote!()
    } else {
        let input_names: Vec<_> = inputs
            .iter()
            .filter_map(|argument| match argument {
                syn::FnArg::Typed(typed) => Some(typed),
                syn::FnArg::Receiver(rec) => {
                    use syn::spanned::Spanned;
                    rec.span().unwrap().error("Don't use `self` on components");
                    None
                }
            })
            .map(|value| {
                let pat = &value.pat;
                quote!(#pat)
            })
            .collect();
        quote!(
            let #struct_name { #(#input_names),* } = self;
        )
    };

    TokenStream::from(quote! {
        #[derive(Debug)]
        #vis struct #struct_name#impl_generics #inputs_block

        impl#impl_generics ::render::Renderable for #struct_name #ty_generics #where_clause {
            fn render(self) -> String {
                #inputs_reading
                #block
            }
        }
    })
}
