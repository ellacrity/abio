use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod internal;
use internal::Derivable;

#[proc_macro_derive(Aligned)]
pub fn derive_aligned(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct or enum
    let name = &input.ident;
    println!("{name:?}");

    // Generate the implementation of the Aligned trait
    let expanded = quote! {
        impl Aligned for #name {}
    };

    // Return the generated implementation as tokens
    TokenStream::from(expanded)
}

#[proc_macro_derive(ReadBytes)]
pub fn derive_read_bytes(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct or enum
    let name = &input.ident;
    println!("{name:?}");

    // Generate the implementation of the Aligned trait
    let expanded = quote! {
        impl Aligned for #name {}
    };

    // Return the generated implementation as tokens
    TokenStream::from(expanded)
}
