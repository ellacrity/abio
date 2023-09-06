#![allow(dead_code)]

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Error, Result};

mod helpers;
use helpers::{Abi, AsBytes, Decode, Marker, Zeroable};
mod traits;

#[proc_macro_derive(Abi)]
pub fn derive_abi(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match gen_marker_trait_impl::<Abi>(&input) {
        Ok(imp) => imp.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(AsBytes)]
pub fn derive_as_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match gen_marker_trait_impl::<AsBytes>(&input) {
        Ok(imp) => imp.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match gen_marker_trait_impl::<Zeroable>(&input) {
        Ok(imp) => imp.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Parse the input AST from the `Decode<T>` trait.
    let expanded = parse_decode_input(&input);

    // Return the generated implementation as tokens
    proc_macro::TokenStream::from(expanded)
}

fn parse_decode_input(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;
    quote!()
}

fn derive_decode_trait(_input: &DeriveInput) -> TokenStream {
    unimplemented!()
}

fn gen_marker_trait_impl<G: Marker>(input: &DeriveInput) -> Result<TokenStream> {
    // Ensure that each field of the type implements the trait
    let mut input = input.clone();
    let trait_name = G::ident(&input);

    let input = G::add_trait_marker(&mut input, &trait_name);

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    _ = G::validate_attributes(&input.data, &input.attrs).map_err(|err| {
        eprintln!("{err:?}");
        Error::new(
            Span::call_site(),
            "Cannot implement this trait for this type due to invalid attribute values.",
        )
    });

    let assertions = match G::asserts(&input) {
        Ok(asserts) => asserts,
        Err(err) => err
            .into_compile_error()
            .to_token_stream(),
    };

    let (trait_impl_extras, trait_impl) = G::trait_impl(&input)?;

    let impl_prefix = if G::is_unsafe(&input) {
        quote! {
            unsafe impl
        }
    } else {
        quote! {
            impl
        }
    };

    let implies_trait = if let Some(implies_trait) = G::fulfills_contract() {
        quote! {
            #impl_prefix #impl_generics #implies_trait for #name #ty_generics #where_clause {}
        }
    } else {
        quote! {}
    };

    let where_clause = if G::requires_where_clause() { where_clause } else { None };

    Ok(quote! {
      #assertions

      #trait_impl_extras

      #impl_prefix #impl_generics #trait_name for #name #ty_generics #where_clause {
        #trait_impl
      }

      #implies_trait
    })
}
