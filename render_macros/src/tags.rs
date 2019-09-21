use crate::element_attributes::ElementAttributes;
use syn::parse::{Parse, ParseStream, Result};

pub struct OpenTag {
    pub name: syn::Ident,
    pub attributes: ElementAttributes,
    pub self_closing: bool,
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let _starts_a_tag = input.parse::<syn::Token![<]>().is_ok();
        let name = input.parse()?;
        let attributes = input.parse::<ElementAttributes>()?;
        let self_closing = input.parse::<syn::Token![/]>().is_ok();
        input.parse::<syn::Token![>]>()?;

        Ok(Self {
            name,
            attributes,
            self_closing,
        })
    }
}

pub struct ClosingTag {
    name: syn::Ident,
}

impl ClosingTag {
    pub fn validate(&self, open_tag: &OpenTag) {
        if self.name != open_tag.name {
            let error_message = format!("Expected closing tag for: <{}>", &open_tag.name);
            self.name.span().unwrap().error(error_message).emit();
        }
    }
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![/]>()?;
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![>]>()?;
        Ok(Self { name })
    }
}
