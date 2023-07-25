//! Module containing utilities and helpers for working with trait bounds

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, token, Ident, Result, Token, TypeParamBound};
use syn::{parse_quote, Error, GenericParam, Generics};

// Add a bound `T: HeapSize` to every type parameter T.
pub fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(::abio::AsBytes));
        }
    }
    generics
}
