#![allow(unused_imports)]
use core::ops::Range;

use proc_macro::{Delimiter as Delimiter1, TokenStream as TokenStream1, TokenStream as TokenTree1};
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Nothing, Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, parse_quote, token, AngleBracketedGenericArguments,
    AttrStyle, Attribute, Data, DataStruct, DataUnion, DeriveInput, Error, Expr, Field, Fields,
    Generics, ImplGenerics, Meta, Path, Result, Token, Type, TypeGenerics, Visibility, WhereClause,
};

mod bounds;

pub trait Contract {
    /// Returns true if implementing the trait requires an `unsafe` declaration.
    fn is_unsafe(input: &DeriveInput) -> bool;

    /// Whether the type's attributes are valid and fulfill the contract for this
    /// trait.
    fn check_attributes(_ty: &Data, _attributes: &[Attribute]) -> Result<()> {
        Ok(())
    }

    /// Returns `true` if the implementation requires a `where` clause.
    fn requires_where_clause() -> bool {
        true
    }
}

pub trait Behaviour {
    fn requires_manual_impl(&self) -> bool;
}

pub trait Generate: Contract {
    /// Fully-qualified identifier emitted as a derived trait for the type.
    fn ident(input: &DeriveInput) -> syn::Path;

    /// Assertions generated to ensure ABI-compatibilty at compile time.
    fn asserts(_input: &DeriveInput) -> Result<TokenStream> {
        Ok(quote!())
    }

    /// Returns the
    fn fulfills_contract() -> Option<TokenStream> {
        None
    }

    /// Emit the source code comprising the trait implementation.
    fn trait_impl(_input: &DeriveInput) -> Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }

    /// Add a trait marker to the generics if it is not already present
    fn add_trait_marker(generics: &mut syn::Generics, trait_name: &syn::Path) {
        // Get each generic type parameter.
        let type_params = generics
            .type_params()
            .map(|param| &param.ident)
            .inspect(|ident| println!("ident: {ident}"))
            .map(|param| {
                parse_quote!(
                  #param: #trait_name
                )
            })
            .collect::<Vec<syn::WherePredicate>>();

        generics.make_where_clause().predicates.extend(type_params);
    }
}

pub struct Abi {
    stream: Box<&'static DeriveInput>,
}

impl Contract for Abi {
    fn is_unsafe(_: &DeriveInput) -> bool {
        true
    }

    fn check_attributes(_ty: &Data, _attributes: &[Attribute]) -> Result<()> {
        Ok(())
    }
}

impl Generate for Abi {
    fn ident(_: &DeriveInput) -> syn::Path {
        syn::parse_quote!(::abio::Abi)
    }

    fn asserts(input: &DeriveInput) -> Result<TokenStream> {
        if let Ok(repr) = parse_repr_attr(&input.attrs) {
            let is_valid = repr.packed == Some(1) || repr.repr == Repr::Transparent;

            if !is_valid && !input.generics.params.is_empty() {
                Error::new_spanned(
                    input
                        .generics
                        .params
                        .first()
                        .expect("AST parser cannot get first generic parameter."),
                    include_str!("../docs/derive_abi_message"),
                );
            }

            match &input.data {
                Data::Struct(_) => {
                    let assert_no_padding = if !is_valid {
                        // generate code to check for padding
                        Some(generate_padding_checks(input)?)
                    } else {
                        None
                    };

                    let path = Self::ident(input);
                    let assert_fields_are_abi_compat = generate_fields_are_trait(input, path)?;

                    Ok(quote! {
                      #assert_no_padding
                      #assert_fields_are_abi_compat
                    })
                }
                Data::Enum(..) => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Enum types cannot derive the `Abi` trait.",
                    ))
                }
                Data::Union(..) => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Union types cannot derive the `Abi` trait.",
                    ))
                }
            }
        } else {
            return Err(Error::new_spanned(
                input.attrs.first().expect("AST parser cannot get first generic parameter."),
                "AST parser cannot get `repr` attribute from this type.",
            ));
        }
    }

    fn trait_impl(_input: &DeriveInput) -> Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }
}

pub struct Decode;

impl Contract for Decode {
    fn is_unsafe(_: &DeriveInput) -> bool {
        false
    }
}

impl Generate for Decode {
    fn ident(_: &DeriveInput) -> syn::Path {
        parse_quote!(::abio::Decode)
    }

    fn fulfills_contract() -> Option<TokenStream> {
        None
    }

    fn asserts(_input: &DeriveInput) -> Result<TokenStream> {
        Ok(quote!())
    }

    fn trait_impl(_input: &DeriveInput) -> Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }
}

pub struct BytesOf;

impl Contract for BytesOf {
    fn is_unsafe(_: &DeriveInput) -> bool {
        true
    }

    fn check_attributes(_ty: &Data, _attributes: &[Attribute]) -> Result<()> {
        Ok(())
    }

    fn requires_where_clause() -> bool {
        true
    }
}

impl Generate for BytesOf {
    fn ident(_input: &DeriveInput) -> syn::Path {
        parse_quote!(::abio::BytesOf)
    }

    fn fulfills_contract() -> Option<TokenStream> {
        None
    }

    fn asserts(input: &DeriveInput) -> Result<TokenStream> {
        if let Ok(repr) = parse_repr_attr(&input.attrs) {
            let is_valid = repr.packed == Some(1) || repr.repr == Repr::Transparent;

            if !is_valid && !input.generics.params.is_empty() {
                Error::new_spanned(
                    input
                        .generics
                        .params
                        .first()
                        .expect("AST parser cannot get first generic parameter."),
                    include_str!("../docs/derive_as_bytes_message"),
                );
            }

            match &input.data {
                Data::Struct(_) => {
                    // let assert_no_padding = if !is_valid {
                    //     // generate code to check for padding
                    //     Some(generate_as_bytes_checks(input)?)
                    // } else {
                    //     None
                    // };

                    let path = Self::ident(input);
                    let assert_fields_are_as_bytes = generate_fields_are_trait(input, path)?;

                    Ok(quote! {
                      #assert_fields_are_as_bytes
                    })
                }
                Data::Enum(..) => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Enum types cannot derive the `Abi` trait.",
                    ))
                }
                Data::Union(..) => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Union types cannot derive the `Abi` trait.",
                    ))
                }
            }
        } else {
            return Err(Error::new_spanned(
                input.attrs.first().expect("AST parser cannot get first generic parameter."),
                "AST parser cannot get `repr` attribute from this type.",
            ));
        }
    }

    fn trait_impl(_input: &DeriveInput) -> Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }
}

pub struct Zeroable;

impl Contract for Zeroable {
    fn is_unsafe(_: &DeriveInput) -> bool {
        true
    }

    fn check_attributes(_ty: &Data, _attributes: &[Attribute]) -> Result<()> {
        Ok(())
    }

    fn requires_where_clause() -> bool {
        true
    }
}

impl Generate for Zeroable {
    fn ident(_input: &DeriveInput) -> syn::Path {
        syn::parse_quote!(::abio::Zeroable)
    }
}

fn get_struct_fields(input: &DeriveInput) -> Result<&Fields> {
    if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        Ok(fields)
    } else {
        Err(Error::new(Span::call_site(), "deriving this trait is only supported for structs"))
    }
}

fn get_fields(input: &DeriveInput) -> Result<Fields> {
    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => Ok(fields.clone()),
        Data::Union(DataUnion { fields, .. }) => Ok(Fields::Named(fields.clone())),
        Data::Enum(_) => {
            Err(Error::new(Span::call_site(), "deriving this trait is not supported for enums"))
        }
    }
}

fn get_field_types<'ast>(fields: &'ast Fields) -> impl Iterator<Item = &'ast Type> + 'ast {
    fields.iter().map(|f| &f.ty)
}

/// Check that a struct has no padding by asserting that the size of the struct
/// is equal to the sum of the size of it's fields
fn generate_padding_checks(input: &DeriveInput) -> Result<TokenStream> {
    let struct_type = &input.ident;
    let span = input.ident.span();
    let fields = get_fields(input)?;

    let mut field_types = get_field_types(&fields);
    let size_sum = if let Some(first) = field_types.next() {
        let size_first = quote_spanned!(span => ::core::mem::size_of::<#first>());
        let size_rest = quote_spanned!(span => #( + ::core::mem::size_of::<#field_types>() )*);

        quote_spanned!(span => #size_first #size_rest)
    } else {
        quote_spanned!(span => 0)
    };

    Ok(quote_spanned! {span => const _: fn() = || {
      #[doc(hidden)]
      struct WithAbiCompatPadding([u8; #size_sum]);
      let _ = ::core::mem::transmute::<#struct_type, WithAbiCompatPadding>;
    };})
}

/// Check that all fields implement a given trait
fn generate_fields_are_trait(input: &DeriveInput, trait_: syn::Path) -> Result<TokenStream> {
    let (impl_generics, _ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = get_fields(input)?;
    let span = input.span();
    let field_types = get_field_types(&fields);
    Ok(quote_spanned! {span => #(const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check #impl_generics () #where_clause {
          fn assert_impl<T: #trait_>() {}
          assert_impl::<#field_types>();
        }
      };)*
    })
}

fn get_ident_from_stream(tokens: TokenStream) -> Option<Ident> {
    match tokens.into_iter().next() {
        Some(TokenTree::Group(group)) => get_ident_from_stream(group.stream()),
        Some(TokenTree::Ident(ident)) => Some(ident),
        _ => None,
    }
}

/// get a simple #[foo(bar)] attribute, returning "bar"
fn get_simple_attr(attributes: &[Attribute], attr_name: &str) -> Option<Ident> {
    for attr in attributes {
        if let (AttrStyle::Outer, Meta::List(list)) = (&attr.style, &attr.meta) {
            if list.path.is_ident(attr_name) {
                if let Some(ident) = get_ident_from_stream(list.tokens.clone()) {
                    return Some(ident);
                }
            }
        }
    }

    None
}

mk_repr! {
  U8 => u8,
  I8 => i8,
  U16 => u16,
  I16 => i16,
  U32 => u32,
  I32 => i32,
  U64 => u64,
  I64 => i64,
  I128 => i128,
  U128 => u128,
  Usize => usize,
  Isize => isize,
}
// where
macro_rules! mk_repr {(
  $(
    $Xn:ident => $xn:ident
  ),* $(,)?
) => (
  #[derive(Clone, Copy, Default, PartialEq)]
  enum Repr {
    #[default]
    Default,
    C,
    Transparent,
    $($Xn),*
  }

  impl Repr {
    fn is_integer(self) -> bool {
      match self {
        Repr::Default | Repr::C | Repr::Transparent => false,
        _ => true,
      }
    }

    fn as_integer_type(self) -> Option<TokenStream> {
      match self {
        Repr::Default | Repr::C | Repr::Transparent => None,
        $(
          Repr::$Xn => Some(quote! { ::core::primitive::$xn }),
        )*
      }
    }
  }


#[derive(Clone, Copy)]
  struct Representation {
    packed: Option<u32>,
    align: Option<u32>,
    repr: Repr,
  }

  impl Default for Representation {
    fn default() -> Self {
      Self { packed: None, align: None, repr: Repr::Default }
    }
  }

  impl syn::parse::Parse for Representation {
    fn parse(input: ParseStream<'_>) -> Result<Representation> {
      let mut ret = Representation::default();
      while !input.is_empty() {
        let keyword = input.parse::<Ident>()?;
        // preëmptively call `.to_string()` *once* (rather than on `is_ident()`)
        let keyword_str = keyword.to_string();
        let new_repr = match keyword_str.as_str() {
          "C" => Repr::C,
          "transparent" => Repr::Transparent,
          "packed" => {
            ret.packed = Some(if input.peek(syn::token::Paren) {
              let contents; parenthesized!(contents in input);
              syn::LitInt::base10_parse::<u32>(&contents.parse()?)?
            } else {
              1
            });
            let _: Option<Token![,]> = input.parse()?;
            continue;
          },
          "align" => {
            let contents; parenthesized!(contents in input);
            ret.align = Some(syn::LitInt::base10_parse::<u32>(&contents.parse()?)?);
            let _: Option<Token![,]> = input.parse()?;
            continue;
          },
        $(
          stringify!($xn) => Repr::$Xn,
        )*
          _ => return Err(input.error("unrecognized representation hint"))
        };
        if ::core::mem::replace(&mut ret.repr, new_repr) != Repr::Default {
          input.error("duplicate representation hint");
        }
        let _: Option<Token![,]> = input.parse()?;
      }
      Ok(ret)
    }
  }

  impl ToTokens for Representation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let repr = match self.repr {
        Repr::Default => None,
        Repr::C => Some(quote!(C)),
        Repr::Transparent => Some(quote!(transparent)),
        $(
          Repr::$Xn => Some(quote!($xn)),
        )*
      };
      let packed = self.packed.map(|p| {
        let lit = syn::LitInt::new(&p.to_string(), Span::call_site());
        quote!(packed(#lit))
      });
      let comma = if packed.is_some() && repr.is_some() {
        Some(quote!(,))
      } else {
        None
      };
      tokens.extend(quote!(
        #[repr( #repr #comma #packed )]
      ));
    }
  }
)}
use mk_repr;

fn parse_repr_attr(attributes: &[Attribute]) -> Result<Representation> {
    attributes
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("repr") {
                Some(attr.parse_args::<Representation>())
            } else {
                None
            }
        })
        .try_fold(Representation::default(), |x, y| {
            let y = y.expect("failed to parse `#[repr(...)]` attribute");
            let repr = match (x.repr, y.repr) {
                (x, Repr::Default) => x,
                (Repr::Default, y) => y,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Conflicting layout representations.",
                    ))
                }
            };

            let packed = match (x.packed, y.packed) {
                (x, None) => x,
                (None, y) => y,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Conflicting layout representations.",
                    ))
                }
            };

            let align = match (x.align, y.align) {
                (x, None) => x,
                (None, y) => y,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Conflicting layout representations.",
                    ))
                }
            };

            Ok(Representation { packed, align, repr })
        })
}
