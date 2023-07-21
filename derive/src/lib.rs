#![allow(dead_code)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod internal;
use internal::{Abi, AsBytes, Derive, Zeroable};

#[proc_macro_derive(Abi)]
pub fn derive_abi(input: TokenStream) -> TokenStream {
    derive_trait::<Abi>(input)
}

#[proc_macro_derive(AsBytes)]
pub fn derive_as_bytes(input: TokenStream) -> TokenStream {
    derive_trait::<AsBytes>(input)
}

#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(input: TokenStream) -> TokenStream {
    derive_trait::<Zeroable>(input)
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

fn derive_trait<D: Derive>(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // Generate the implementation of the Decodable trait
    match derive_trait_inner::<D>(input) {
        Ok(ts) => TokenStream::from(ts),
        Err(err) => err.throw_with_span(Span::mixed_site()).into(),
    }
}

/// Parse the syntax tree associated with a `Decode` implementation.
///
/// ```ignore
/// pub trait Decode<E: Endian>: Abi {
///     /// Offset type used to index into the source.
///     type Offset;
///
///     /// Decodes a concrete type `T` from an immutable reference to `self`.
///     ///
///     /// # Errors
///     ///
///     /// Returns an error if the operation fails due to a size mismatch or misaligned
///     /// read.
///     fn decode(source: &[u8], offset: Self::Offset, endian: E) -> Result<Self>;
/// }
/// ```
fn parse_decode_input(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let mut generator = virtue::generate::Generator::with_name("Decode");
    generator.r#impl().modify_generic_constraints(|generics, constraints| {
        for generic in generics.iter_generics() {
            if let Err(err) = constraints.push_constraint(generic, "Decode") {
                err.throw_with_span(Span::mixed_site());
            }
        }
    });

    println!("Generating code for {name}");
    // let expanded = quote! {
    //     impl #{name}<E: Endian>: Abi {
    //         type Offset = Span;
    //     }
    // };
    // TokenStream::from(expanded)
    generator.finish().expect("failed to generate code for Decode trait.")
}

fn derive_decode_trait(_input: &DeriveInput) -> TokenStream {
    todo!()
}

fn derive_trait_inner<D: Derive>(
    mut input: DeriveInput,
) -> virtue::Result<proc_macro2::TokenStream> {
    // Enforce Abi to be implemented on all
    let trait_name = D::ident(&input);
    add_trait_marker(&mut input.generics, &trait_name);

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    D::check_attributes(&input.data, &input.attrs)
        .map_err(|_| virtue::Error::ExpectedIdent(Span::call_site()))?;

    let asserts = D::asserts(&input).expect("assert failed for type: {name}");
    let (trait_impl_extras, trait_impl) =
        D::trait_impl(&input).map_err(|_| virtue::Error::ExpectedIdent(Span::call_site()))?;

    let impl_prefix = if D::is_unsafe(&input) {
        quote! {
            unsafe impl
        }
    } else {
        quote! {
            impl
        }
    };

    let implies_trait = if let Some(implies_trait) = D::fulfills_contract() {
        quote! {
            #impl_prefix #impl_generics #implies_trait for #name #ty_generics #where_clause {}
        }
    } else {
        quote! {}
    };

    let where_clause = if D::requires_where_clause() { where_clause } else { None };

    Ok(quote! {
      #asserts

      #trait_impl_extras

      #impl_prefix #impl_generics #trait_name for #name #ty_generics #where_clause {
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
