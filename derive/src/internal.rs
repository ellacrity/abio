#![allow(unused_imports)]
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{DeriveInput, Result, *};

macro_rules! bail {
    ($msg:expr $(,)?) => {
        return Err(Error::new(Span::call_site(), &$msg[..]))
    };

    ( $msg:expr => $span_to_blame:expr $(,)? ) => {
        return Err(Error::new_spanned(&$span_to_blame, $msg))
    };
}

pub trait Derivable {
    fn ident(input: &DeriveInput) -> Result<syn::Path>;
    fn implies_trait() -> Option<TokenStream> {
        None
    }
    fn asserts(_input: &DeriveInput) -> Result<TokenStream> {
        Ok(quote!())
    }
    fn check_attributes(_ty: &Data, _attributes: &[Attribute]) -> Result<()> {
        Ok(())
    }
    fn trait_impl(_input: &DeriveInput) -> Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }
    fn requires_where_clause() -> bool {
        true
    }
}

pub struct Aligned;

pub struct Decodable;

pub struct Zeroable;
