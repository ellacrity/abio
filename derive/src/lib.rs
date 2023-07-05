use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

#[proc_macro_derive(Aligned)]
pub fn derive_aligned_trait(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    println!("{ast:#?}");

    let generated = quote! {
        #{ast}
    };

    TokenStream::from(generated)
}
