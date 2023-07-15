// #![feature(const_cstr_from_ptr)]
#![allow(dead_code)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

mod internal;
use internal::{AbiMarker, DecodeImpl, MarkerTrait, SourceImpl, ZeroableMarker};

#[proc_macro_derive(Abi)]
pub fn derive_abi(input: TokenStream) -> TokenStream {
    generate_marker_trait::<AbiMarker>(input)
}

#[proc_macro_derive(Source)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    generate_marker_trait::<SourceImpl>(input)
}

#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(input: TokenStream) -> TokenStream {
    generate_marker_trait::<ZeroableMarker>(input)
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // Parse the input AST from the `Decode<T>` trait.
    let expanded = parse_decode_input(&input);

    // Return the generated implementation as tokens
    TokenStream::from(expanded)
}

fn generate_marker_trait<T: MarkerTrait>(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // Generate the implementation of the Decodable trait
    match derive_marker_trait_inner::<T>(input) {
        Ok(ts) => TokenStream::from(ts),
        Err(err) => err.into_compile_error().into(),
    }
}

fn parse_decode_input(_input: &DeriveInput) -> TokenStream {
    let expanded = quote! {};
    TokenStream::from(expanded)
}

fn derive_decode_trait(_input: &DeriveInput) -> TokenStream {
    todo!()
}

fn derive_marker_trait_inner<M: MarkerTrait>(
    mut input: DeriveInput,
) -> Result<proc_macro2::TokenStream> {
    // Enforce Abi to be implemented on all
    let trait_ = M::ident(&input)?;
    add_trait_marker(&mut input.generics, &trait_);

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    M::check_attributes(&input.data, &input.attrs)?;
    let asserts = M::asserts(&input)?;
    let (trait_impl_extras, trait_impl) = M::trait_impl(&input)?;

    let impl_prefix = if M::is_unsafe(&input) {
        quote! {
            unsafe impl
        }
    } else {
        quote! {
            impl
        }
    };

    let implies_trait = if let Some(implies_trait) = M::fulfills_contract() {
        quote! {
            #impl_prefix #impl_generics #implies_trait for #name #ty_generics #where_clause {}
        }
    } else {
        quote! {}
    };

    let where_clause = if M::requires_where_clause() { where_clause } else { None };

    Ok(quote! {
      #asserts

      #trait_impl_extras

      #impl_prefix #impl_generics #trait_ for #name #ty_generics #where_clause {
        #trait_impl
      }

      #implies_trait
    })
}

/// Add a trait marker to the generics if it is not already present
fn add_trait_marker(generics: &mut syn::Generics, trait_name: &syn::Path) {
    // Get each generic type parameter.
    let type_params = generics
        .type_params()
        .map(|param| &param.ident)
        .map(|param| {
            syn::parse_quote!(
              #param: #trait_name
            )
        })
        .collect::<Vec<syn::WherePredicate>>();

    generics.make_where_clause().predicates.extend(type_params);
}
