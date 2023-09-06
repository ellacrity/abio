use core::iter::Peekable;
use core::{ops, option};

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::{AttrStyle, Attribute, Meta};

/// Parses the next [`Ident`][`syn::Ident`] type from the token trees.
pub fn parse_next_ident(tokens: proc_macro2::TokenStream) -> Option<syn::Ident> {
    match tokens.into_iter().next() {
        Some(TokenTree::Group(group)) => match parse_next_ident(group.stream()) {
            Some(name) => Some(name),
            None => return None,
        },
        Some(TokenTree::Ident(ident)) => Some(ident),
        _ => return None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attr(syn::Attribute);

impl Attr {
    pub fn path(&self) -> &syn::Path {
        self.0.path()
    }
}

/// get a simple #[foo(bar)] attribute, returning "bar"
pub fn parse_attr_path(attributes: &[Attribute], attr_name: &str) -> Option<syn::Ident> {
    while let Some((idx, attr)) = attributes
        .iter()
        .enumerate()
        .next()
    {
        let mut next_attr = attr;
        match (&attr.style, &attr.meta) {
            (AttrStyle::Outer, Meta::List(list)) if list.path.is_ident(attr_name) => {
                match parse_next_ident(list.to_token_stream()) {
                    Some(ident) => return Some(ident),
                    None => continue,
                }
            }
            _ => return None,
        };
    }

    None
}

impl ops::Deref for Attr {
    type Target = syn::Attribute;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Attr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn is_matching_attr(attribute: &Attribute, other: &str) -> Option<syn::Ident> {
    if attribute.path().is_ident(other) {
        Some(format_ident!("{other}"))
    } else {
        None
    }
}

pub fn make_compiler_error(message: &str) -> TokenStream {
    syn::Error::new(Span::call_site(), message)
        .into_compile_error()
        .into()
}

#[doc(hidden)]
pub trait IntoInner<T> {
    fn into_inner(self) -> T;
}

#[doc(hidden)]
pub trait AsInner<T: ?Sized> {
    fn as_inner(&self) -> &T;
}

#[doc(hidden)]
pub trait AsInnerMut<T: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut T;
}

#[doc(hidden)]
pub trait FromInner<T: ?Sized> {
    fn from_inner(inner: T) -> Self;
}
