use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::token::Plus;
use syn::{parse_quote, DeriveInput, TypeParamBound};

mod abi;
pub use abi::Abi;

mod as_bytes;
pub use as_bytes::AsBytes;

mod zeroable;
use syn::punctuated::Punctuated;
pub use zeroable::Zeroable;

/// Trait defining the basic and essential contract that encompasses a traits'
pub trait Contract {
    /// Returns true if implementing the trait requires an `unsafe` declaration.
    fn is_unsafe(input: &DeriveInput) -> bool;

    /// Whether the type's attributes are valid and fulfill the contract for this
    /// trait.
    fn validate_attributes(_ty: &syn::Data, _attributes: &[syn::Attribute]) -> syn::Result<()> {
        Ok(())
    }

    /// Returns `true` if the implementation requires a `where` clause.
    fn requires_where_clause() -> bool {
        true
    }
}

pub struct BoundedField(Ident, Punctuated<TypeParamBound, Plus>);

/// Trait to define a marker trait in source code, or syntax token trees.
pub trait Marker: Contract {
    /// Fully-qualified identifier emitted as a derived trait for the type.
    fn ident(input: &DeriveInput) -> syn::Path;

    /// Assertions generated to ensure ABI-compatibilty at compile time.
    fn asserts(_input: &DeriveInput) -> syn::Result<TokenStream> {
        Ok(quote! {
            const: fn ||
        })
    }

    /// Returns the
    fn fulfills_contract() -> Option<TokenStream> {
        None
    }

    /// Emit the source code comprising the trait implementation.
    fn trait_impl(_input: &DeriveInput) -> syn::Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }

    /// Add a trait marker to the generics if it is not already present
    fn add_trait_marker(input: &mut syn::DeriveInput, trait_name: &syn::Path) -> syn::DeriveInput {
        // Get each generic type parameter.
        let type_params = input
            .generics
            .type_params_mut()
            .map(|param| {
                let param_ident = &param.ident;
                dbg!(&param);
                parse_quote!(
                  #param_ident: #trait_name
                )
            })
            .collect::<Vec<syn::WherePredicate>>();

        input
            .generics
            .make_where_clause()
            .predicates
            .extend(type_params);
        input.clone()
    }
}
