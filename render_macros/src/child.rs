use crate::element::Element;
use proc_macro2::{Literal, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};

pub enum Child {
    Element(Element),
    RawBlock(syn::Block),
    Literal(String),
}

impl ToTokens for Child {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Element(element) => element.to_tokens(tokens),
            Self::RawBlock(block) => {
                let ts = if block.stmts.len() == 1 {
                    let first = &block.stmts[0];
                    quote!(#first)
                } else {
                    quote!(#block)
                };
                ts.to_tokens(tokens);
            }
            Self::Literal(s) => {
                let ls = TokenTree::Literal(Literal::string(&s));
                quote!(#ls).to_tokens(tokens);
            }
        }
    }
}

// HACK until the span start/end methods are available in stable
fn span_pos(s: &proc_macro2::Span) -> (usize, usize) {
    let d = format!("{:?}", s);
    let s = d.find('(').unwrap();
    let e = d.find('.').unwrap();
    return (
        d[s + 1..e].parse().unwrap(),
        d[e + 2..d.len() - 1].parse().unwrap(),
    );
}

impl Parse for Child {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(el) = input.parse() {
            return Ok(Self::Element(el));
        }
        if let Ok(bl) = input.parse() {
            return Ok(Self::RawBlock(bl));
        }
        let s = input.step(|cursor| {
            let mut s = String::new();
            let mut prev_end = 0;
            let mut skip_space = true;
            let mut rest = *cursor;
            while let Some((tt, next)) = rest.token_tree() {
                // stop, we got a braced block
                if let TokenTree::Group(g) = &tt {
                    if g.delimiter() == proc_macro2::Delimiter::Brace {
                        break;
                    }
                }
                // stop, we got a open or close tag
                if let TokenTree::Punct(p) = &tt {
                    if p.as_char() == '<' {
                        break;
                    }
                }
                let (span_start, span_end) = span_pos(&tt.span());
                if !skip_space && prev_end != span_start {
                    s.push(' ');
                }
                prev_end = span_end;
                skip_space = false;
                s.push_str(&tt.to_string());
                rest = next;
            }
            Ok((s, rest))
        })?;
        Ok(Self::Literal(s))
    }
}
