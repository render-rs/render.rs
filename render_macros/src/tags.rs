use crate::element_attributes::ElementAttributes;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

pub struct OpenTag {
    pub name: syn::Path,
    pub attributes: ElementAttributes,
    pub self_closing: bool,
}

fn name_or_fragment(maybe_name: Result<syn::Path>) -> syn::Path {
    maybe_name.unwrap_or_else(|_| {
        syn::parse_str::<syn::Path>("::render::Fragment").unwrap()
    })
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::Token![<]>()?;
        let maybe_name = syn::Path::parse_mod_style(input);
        let attributes = input.parse::<ElementAttributes>()?;
        let self_closing = input.parse::<syn::Token![/]>().is_ok();
        input.parse::<syn::Token![>]>()?;

        let name = name_or_fragment(maybe_name);

        Ok(Self {
            name,
            attributes,
            self_closing,
        })
    }
}

pub struct ClosingTag {
    name: syn::Path,
}

impl ClosingTag {
    pub fn validate(&self, open_tag: &OpenTag) {
        let open_tag_path = &open_tag.name;
        let open_tag_path_str = quote!(#open_tag_path).to_string();
        let self_path = &self.name;
        let self_path_str = quote!(#self_path).to_string();
        if self_path_str != open_tag_path_str {
            let error_message = format!("Expected closing tag for: <{}>", &open_tag_path_str);
            self.name.span().unwrap().error(error_message).emit();
        }
    }
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![/]>()?;
        let maybe_name = input.parse::<syn::Path>();
        input.parse::<syn::Token![>]>()?;
        Ok(Self {
            name: name_or_fragment(maybe_name),
        })
    }
}
