#![feature(proc_macro_diagnostic, proc_macro_hygiene)]

// TODO: Extract a `Children` struct that can implement `Parse` and `ToTokens`:
// - `Parse` will do the nasty things inside `Element`
// - `ToTokens` will do the trick with the tuples

extern crate proc_macro;

mod element_attribute;

use element_attribute::ElementAttribute;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;

struct Element {
    name: syn::Ident,
    attributes: HashSet<ElementAttribute>,
    children: Vec<RenderableItem>,
}

enum RenderableItem {
    Element(Element),
    RawBlock(syn::Block),
}

impl ToTokens for RenderableItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Element(element) => element.to_tokens(tokens),
            Self::RawBlock(block) => {
                let ts = quote! { #block };
                ts.to_tokens(tokens);
            }
        }
    }
}

impl Parse for RenderableItem {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.parse::<Element>() {
            Ok(element) => Ok(Self::Element(element)),
            Err(_) => {
                let block = input.parse::<syn::Block>()?;
                Ok(Self::RawBlock(block))
            }
        }
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attributes: HashSet<ElementAttribute> = HashSet::new();
        let _starts_a_tag = input.parse::<syn::Token![<]>().is_ok();

        let name = input.parse()?;

        while input.peek(syn::Ident) {
            if let Ok(attribute) = input.parse::<ElementAttribute>() {
                if attributes.contains(&attribute) {
                    let error_message = format!(
                        "There is a previous definition of the {} attribute",
                        attribute.ident()
                    );
                    attribute
                        .ident()
                        .span()
                        .unwrap()
                        .warning(error_message)
                        .emit();
                }
                attributes.insert(attribute);
            }
        }

        let can_have_contents = input.parse::<syn::Token![/]>().is_err();
        input.parse::<syn::Token![>]>()?;

        let mut children = vec![];

        if can_have_contents {
            while !input.peek(syn::Token![<]) || !input.peek2(syn::Token![/]) {
                if let Ok(child) = input.parse::<RenderableItem>() {
                    children.push(child);
                }
            }

            // parse closing
            input.parse::<syn::Token![<]>()?;
            input.parse::<syn::Token![/]>()?;
            let closing_name: syn::Ident = input.parse()?;
            if closing_name != name {
                let error_message = format!("Expected closing tag for: <{}>", &name);
                closing_name.span().unwrap().error(error_message).emit();
            }
            input.parse::<syn::Token![>]>()?;
        }

        Ok(Element {
            name,
            attributes,
            children,
        })
    }
}

impl Element {
    pub fn is_custom_element(&self) -> bool {
        let name = self.name.to_string();
        let first_letter = name.get(0..1).unwrap();
        first_letter.to_uppercase() == first_letter
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Element { name, children, .. } = self;

        let children_quotes: Vec<_> = children
            .iter()
            .map(|child| {
                quote! { #child }
            })
            .collect();
        let children_tuple = match children_quotes.len() {
            0 => quote! { Option::<()>::None },
            1 => quote! { Some(#(#children_quotes)*) },
            _ => {
                let mut iter = children_quotes.iter();
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                let tuple_of_tuples = iter.fold(
                    quote!((#first, #second)),
                    |renderable, current| quote!((#current, #renderable)),
                );

                quote! { Some(#tuple_of_tuples) }
            }
        };

        let declaration = if self.is_custom_element() {
            let mut attrs: Vec<_> = self
                .attributes
                .iter()
                .map(|attribute| {
                    let ident = attribute.ident();
                    let value = attribute.value_tokens();

                    quote! {
                        #ident: #value
                    }
                })
                .collect();

            if children_quotes.len() > 0 {
                attrs.push(quote! {
                    children: #children_tuple
                });
            }

            if attrs.len() == 0 {
                quote! { #name }
            } else {
                quote! {
                    #name {
                        #(#attrs),*
                    }
                }
            }
        } else {
            let attrs: Vec<_> = self
                .attributes
                .iter()
                .map(|attribute| {
                    let ident = attribute.ident();
                    let value = attribute.value_tokens();

                    quote! {
                        hm.insert(stringify!(#ident), #value);
                    }
                })
                .collect();
            let attributes_value = if self.attributes.len() == 0 {
                quote!(None)
            } else {
                quote! {{
                    let mut hm = std::collections::HashMap::<&str, &str>::new();
                    #(#attrs)*
                    Some(hm)
                }}
            };
            quote! {
                ::render::SimpleElement {
                    tag_name: stringify!(#name),
                    attributes: #attributes_value,
                    contents: #children_tuple,
                }
            }
        };
        declaration.to_tokens(tokens);
    }
}

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
