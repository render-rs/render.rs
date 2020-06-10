use crate::child::Child;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};

#[derive(Default)]
pub struct Children {
    pub nodes: Vec<Child>,
}

impl Children {
    pub fn new(nodes: Vec<Child>) -> Self {
        Children { nodes }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn as_option_of_tuples_tokens(&self) -> proc_macro2::TokenStream {
        let children_quotes: Vec<_> = self
            .nodes
            .iter()
            .map(|child| {
                quote! { #child }
            })
            .collect();

        match children_quotes.len() {
            0 => quote! { Option::<()>::None },
            1 => quote! { Some(#(#children_quotes),*) },
            _ => {
                let mut iter = children_quotes.iter();

                let first = iter.next().unwrap();
                let second = iter.next().unwrap();

                let tuple_of_tuples = iter.fold(
                    quote!((#first, #second)),
                    |renderable, current| quote!((#renderable, #current)),
                );

                quote! { Some(#tuple_of_tuples) }
            }
        }
    }
}

impl Parse for Children {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = vec![];

        while !input.peek(syn::Token![<]) || !input.peek2(syn::Token![/]) {
            let child = input.parse::<Child>()?;
            nodes.push(child);
        }

        Ok(Self::new(nodes))
    }
}

impl ToTokens for Children {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.as_option_of_tuples_tokens().to_tokens(tokens);
    }
}
