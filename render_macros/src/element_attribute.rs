use quote::quote;
use std::hash::{Hash, Hasher};
use syn::parse::{Parse, ParseStream, Result};

pub enum ElementAttribute {
    Punned(syn::Ident),
    WithValue(syn::Ident, syn::Block),
}

impl ElementAttribute {
    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Punned(ident) | Self::WithValue(ident, _) => ident,
        }
    }

    pub fn value_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Self::WithValue(_, value) => quote!(#value),
            Self::Punned(ident) => quote!(#ident),
        }
    }
}

impl PartialEq for ElementAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.ident() == other.ident()
    }
}

impl Eq for ElementAttribute {}

impl Hash for ElementAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self.ident(), state)
    }
}

impl Parse for ElementAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let not_punned = input.peek(syn::Token![=]);

        if !not_punned {
            return Ok(Self::Punned(name));
        }

        input.parse::<syn::Token![=]>()?;
        let value = input.parse::<syn::Block>()?;

        Ok(Self::WithValue(name, value))
    }
}
