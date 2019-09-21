#![feature(proc_macro_diagnostic, proc_macro_hygiene)]

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

/// Render a component tree to an HTML string
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let el = proc_macro2::TokenStream::from(rsx(input));
    let result = quote! { ::render::Renderable::render(#el) };
    TokenStream::from(result)
}

/// Generate a renderable component tree
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as Element);
    let result = quote! { #el };
    TokenStream::from(result)
}
