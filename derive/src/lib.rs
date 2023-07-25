#![allow(dead_code)]

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Error, Result};

mod helpers;
use helpers::{Abi, AsBytes, Generate, Zeroable};

const ABIO_DEBUG: &str = "ABIO_DEBUG";

#[proc_macro_derive(Abi)]
pub fn derive_abi(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_marker_trait_inner::<Abi>(input)
}

#[proc_macro_derive(AsBytes)]
pub fn derive_as_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_marker_trait_inner::<AsBytes>(input)
}

#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_marker_trait_inner::<Zeroable>(input)
}

fn derive_marker_trait_inner<G: Generate>(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate_trait_impl::<G>(input).unwrap_or_else(|e| e.into_compile_error()).into()
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // Parse the input AST from the `Decode<T>` trait.
    let expanded = parse_decode_input(&input);

    // Return the generated implementation as tokens
    TokenStream::from(expanded).into()
}

fn inspect_input(tts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(tts as DeriveInput);
    println!("{parsed:#?}");
    proc_macro::TokenStream::from(parsed.to_token_stream())
}

fn parse_decode_input(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", input.ident);
    quote! {
        #name
    }
}

fn derive_decode_trait(_input: &DeriveInput) -> TokenStream {
    todo!()
}

fn generate_trait_impl<G: Generate>(mut input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    // Enforce Abi to be implemented on all
    let trait_name = G::ident(&input);
    G::add_trait_marker(&mut input.generics, &trait_name);

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    G::check_attributes(&input.data, &input.attrs).map_err(|err| {
        eprintln!("{err:?}");
        Error::new(
            Span::call_site(),
            "Cannot implement this trait for this type due to invalid attribute values.",
        )
    })?;

    let assertions = match G::asserts(&input) {
        Ok(asserts) => asserts,
        Err(err) => err.into_compile_error().to_token_stream(),
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
