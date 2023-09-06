#![allow(unused_imports)]
use core::ops::{BitOr, Range};
use core::sync::atomic::AtomicBool;

use proc_macro::{Delimiter as Delimiter1, TokenStream as TokenStream1, TokenStream as TokenTree1};
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Nothing, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, parse_quote, token, AngleBracketedGenericArguments,
    AttrStyle, Data, DataStruct, DataUnion, DeriveInput, Error, Expr, Field, Fields, Generics,
    ImplGenerics, Meta, Path, Result, Token, Type, TypeGenerics, Visibility, WhereClause,
};

mod markers;
pub use markers::{Abi, AsBytes, BoundedField, Contract, Marker, Zeroable};

mod general;
pub use general::Decode;

pub struct Properties {
    is_unsafe: bool,
    repr: Repr,
}

impl Contract for Abi {
    fn is_unsafe(_: &DeriveInput) -> bool {
        true
    }

    fn validate_attributes(_ty: &Data, _attributes: &[syn::Attribute]) -> Result<()> {
        Ok(())
    }
}

impl Marker for Abi {
    fn ident(_: &DeriveInput) -> syn::Path {
        syn::parse_quote!(::abio::Abi)
    }

    fn asserts(input: &DeriveInput) -> Result<TokenStream> {
        if let Ok(layout) = ComptimeLayout::parse_repr_attr(&input.attrs) {
            let is_valid = layout.packed == Some(1) || layout.repr == Repr::Transparent;

            let punctuated = &input.generics.params;
            println!("input.generic.params: {punctuated:?}");
            if !is_valid && !punctuated.is_empty() {
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
                    Err(Error::new(Span::call_site(), "Enum types cannot derive the `Abi` trait."))
                }
                Data::Union(..) => {
                    Err(Error::new(Span::call_site(), "Union types cannot derive the `Abi` trait."))
                }
            }
        } else {
            return Err(Error::new_spanned(
                input
                    .attrs
                    .first()
                    .expect("AST parser cannot get first generic parameter."),
                "AST parser cannot get `repr` attribute from this type.",
            ));
        }
    }

    fn trait_impl(_input: &DeriveInput) -> Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }
}

impl Contract for AsBytes {
    fn is_unsafe(_: &DeriveInput) -> bool {
        true
    }

    fn validate_attributes(_ty: &Data, _attributes: &[syn::Attribute]) -> Result<()> {
        Ok(())
    }

    fn requires_where_clause() -> bool {
        true
    }
}

impl Marker for AsBytes {
    fn ident(_input: &DeriveInput) -> syn::Path {
        parse_quote!(::abio::AsBytes)
    }

    fn fulfills_contract() -> Option<TokenStream> {
        None
    }

    fn asserts(input: &DeriveInput) -> Result<TokenStream> {
        if let Ok(layout) = ComptimeLayout::parse_repr_attr(&input.attrs) {
            let is_valid = fun_name(layout);

            let punctuated = &input.generics.params;
            println!("input.generic.params: {punctuated:?}");

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
                    let path = Self::ident(input);
                    let assert_fields_are_as_bytes = generate_fields_are_trait(input, path)?;

                    Ok(quote! {
                      #assert_fields_are_as_bytes
                    })
                }
                Data::Enum(..) => {
                    Err(Error::new(Span::call_site(), "Enum types cannot derive the `Abi` trait."))
                }
                Data::Union(..) => {
                    Err(Error::new(Span::call_site(), "Union types cannot derive the `Abi` trait."))
                }
            }
        } else {
            return Err(Error::new_spanned(
                input
                    .attrs
                    .first()
                    .expect("AST parser cannot get first generic parameter."),
                "AST parser cannot get `repr` attribute from this type.",
            ));
        }
    }

    fn trait_impl(_input: &DeriveInput) -> Result<(TokenStream, TokenStream)> {
        Ok((quote!(), quote!()))
    }
}

fn fun_name(layout: ComptimeLayout) -> bool {
    layout.packed == Some(1) || layout.repr == Repr::Transparent
}

impl Contract for Zeroable {
    fn is_unsafe(_: &DeriveInput) -> bool {
        true
    }

    fn validate_attributes(_ty: &Data, _attributes: &[syn::Attribute]) -> Result<()> {
        Ok(())
    }

    fn requires_where_clause() -> bool {
        true
    }
}

impl Marker for Zeroable {
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

fn get_field_types(fields: &Fields) -> impl Iterator<Item = &Type> {
    fields.iter().map(|f| &f.ty)
}

/// Check that a struct has no padding by asserting that the size of the struct
/// is equal to the sum of the size of it's fields
fn generate_padding_checks(input: &DeriveInput) -> Result<TokenStream> {
    let struct_type = &input.ident;
    let span = input.ident.span();
    let fields = get_fields(input)?;

    let mut field_types = get_field_types(&fields);

    let type_size = if let Some(first) = field_types.next() {
        let size_first = quote_spanned!(span => ::core::mem::size_of::<#first>());
        let size_rest = quote_spanned!(span => #( + ::core::mem::size_of::<#field_types>() )*);

        quote_spanned!(span => #size_first #size_rest)
    } else {
        quote_spanned!(span => 0)
    };

    Ok(quote_spanned! {span => const _: fn() = || {
      #[doc(hidden)]
      struct WithAbiCompatPadding([u8; #type_size]);
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

/// get a simple #[foo(bar)] attribute, returning "bar".
pub fn get_simple_attr(attributes: &[syn::Attribute], attr_name: &str) -> Option<Ident> {
    for attr in attributes {
        if let Some(value) = attr_match_recursive(attr, attr_name) {
            return Some(value);
        } else {
            continue;
        }
    }

    None
}

/// Check a single attribute path to see if it matches the expected ident.
pub fn attr_match_recursive(attr: &syn::Attribute, attr_name: &str) -> Option<Ident> {
    if let (AttrStyle::Outer, Meta::List(list)) = (&attr.style, &attr.meta) {
        if list.path.is_ident(attr_name) {
            if let Some(ident) = get_ident_from_stream(list.tokens.clone()) {
                return Some(ident);
            }
        }
    }
    None
}

/// Marker trait to define valid integer primitives that may be used within
/// `#[repr($int)]` style attributes.
///
/// # Safety
///
/// TODO: Safety comment
pub(crate) unsafe trait Integer {}
macro_rules! impl_integer_for_primitives {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Integer for $ty {}
        )*
    }
}

impl_integer_for_primitives! {
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
}

pub(crate) enum StandardLayouts {
    Rust,
    Transparent,
    C,
    Integer(Box<dyn Integer>),
}

macro_rules! impl_repr_type {
  (
    $(#[$attr:meta])*
    @$enum_ty:ident: $($name:ident => $int:ident),* $(,)?
  ) => {

        /// Type to represent the possible layout hints, indicated by the `#[repr(...)]`
        /// attribute.
        $(#[$attr])*
        #[derive(Default)]
        enum $enum_ty {
            #[doc = "Default representation used by the compiler when there is no `repr` attribute."]
            #[doc = ""]
            #[doc = "# Warning"]
            #[doc = ""]
            #[doc = "Using this layout representation allows the compiler to freely rearrange"]
            #[doc = "the layout of your type, which may result in the addition of padding bytes"]
            #[doc = "or other incompatible layouts with the ABI defined by the [`abio`][crate] crate."]
            #[default]
            Rust,

            #[doc = "Representation hint that asks compiler to treat the data as if it were structured"]
            #[doc = "like C. Types that are `#[repr(C)]` should be fully C-compatible."]
            C,

            /// Layout hint to indicate to the compiler that the type shares an
            /// identical layout.
            ///
            /// This representation is useful for FFI types since it encourages
            /// creating safer abstractions through newtypes with added or extended
            /// functionality.
            Transparent,

            $(

                $name,
            )*
        }

        impl Clone for $enum_ty {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl Copy for $enum_ty {}
        impl ::core::fmt::Debug for $enum_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self, f)
            }
        }
    };
    (@$enum_ty:ident: $($outer:ident => $inner:ident),* $(,)?) => {
        $(
            stringify!($inner) => $enum_ty::$outer,
        )*
    };
    () => {};
}

impl_repr_type!(
    #[derive(Eq, Ord, PartialEq, PartialOrd)]
    @Repr:
    U8 => u8,
    U16 => u16,
    U32 => u32,
    U64 => u64,
    U128 => u128,
    Usize => usize,
    I8 => i8,
    I16 => i16,
    I32 => i32,
    I64 => i64,
    I128 => i128,
    Isize => isize,

);

pub struct Packed(Option<u32>);

pub struct Align(Option<u32>);

pub struct C(AtomicBool);

pub struct Transparent(bool);

#[derive(Debug, PartialEq)]
pub struct ComptimeLayout {
    /// The type is decorated with `align`, increasing its alignment requirements.
    align: Option<u32>,
    /// The type uses a packed layout, decreasing alignment requirements for the
    /// type.
    packed: Option<u32>,
    /// [`C`][c-layout]-ABI compiler hint regarding type layout.
    c_layout: bool,
    /// Whether the `transparent` hint exists on the type.
    transparent: bool,
    repr: Repr,
}

impl ComptimeLayout {}

impl Default for ComptimeLayout {
    fn default() -> Self {
        Self {
            packed: None,
            align: None,
            repr: Repr::Rust,
            c_layout: false,
            transparent: false,
        }
    }
}

impl ComptimeLayout {
    fn new(
        align: Option<u32>,
        packed: Option<u32>,
        c_layout: bool,
        transparent: bool,
        repr: Repr,
    ) -> Self {
        Self { align, packed, c_layout, transparent, repr }
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.packed == Some(1) || self.repr == Repr::Transparent
    }

    pub(crate) fn peel_packed(self) -> u32 {
        self.packed.unwrap_or(0)
    }

    pub(crate) fn peel_align(self) -> u32 {
        self.align.unwrap_or(0)
    }

    pub(crate) fn is_c_layout(&self) -> bool {
        self.c_layout
    }

    pub(crate) fn is_transparent(&self) -> bool {
        self.transparent
    }

    /// Parses the `repr` attribute from the list of attributes in the source code
    /// for the target type.
    ///
    /// # Accumulator / Fold Method
    ///    
    /// By choosing to initialize a default, legal representation, we can ensure we
    /// start with a valid layout. Using this instance, we can declartively parse
    /// related attributes and their associated values and obtain all relevant
    /// compiler hints.
    ///
    /// The final instance includes a `ComptimeLayout` instance with its `repr`
    /// attribute parsed with graceful failures for any unsupported layout type..
    pub(crate) fn parse_repr_attr(attributes: &[syn::Attribute]) -> Result<Self> {
        attributes
            .iter()
            .filter_map(|attr| parse_attr_repr(attr, "repr").ok())
            .try_fold(Self::default(), |layout, input| {
                let repr = layout.parse_repr(&input)?;
                let packed = layout.parse_packed(&input)?;
                let align = layout.parse_align(&input)?;

                Ok(Self {
                    align: Some(align),
                    packed: Some(packed),
                    repr,
                    ..Self::default()
                })
            })
    }

    fn parse_repr(&self, input: &Self) -> Result<Repr> {
        Ok(match (self.repr, input.repr) {
            (lhs, Repr::Rust) => lhs,
            (Repr::Rust, rhs) => rhs,
            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "Compiler hints contain conflicting layout representations.",
                ))
            }
        })
    }

    fn parse_align(self, input: &Self) -> Result<u32> {
        match (self.align, input.align) {
            (Some(curr), None) => Ok(curr),
            (None, Some(input)) => Ok(input),
            _ => Err(Error::new(
                Span::call_site(),
                "Compiler hints contain conflicting layout representations.",
            )),
        }
    }

    fn with_align(self, input: &Self) -> Result<Self> {
        let align = match (self.align, input.align) {
            (Some(curr), None) => curr,
            (None, Some(input)) => input,
            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "Compiler hints contain conflicting layout representations.",
                ))
            }
        };
        Ok(Self {
            align: self
                .align
                .map(|curr| curr | align),
            ..self
        })
    }

    fn parse_packed(&self, input: &Self) -> Result<u32> {
        match (self.packed, input.packed) {
            (Some(curr), None) => Ok(curr),
            (None, Some(input)) => Ok(input),
            _ => Err(Error::new(
                Span::call_site(),
                "Compiler hints contain conflicting layout representations.",
            )),
        }
    }
}

macro_rules! impl_comptime_layout {
    (@$parent:ident with $enum_ty:ident: $($outer:ident => $inner:ident),* $(,)?) => {
        impl ::syn::parse::Parse for $parent {
        fn parse(input: ::syn::parse::ParseStream<'_>) -> Result<$parent> {
            let mut ret = $parent::default();
            while !input.is_empty() {
                let keyword = input.parse::<::syn::Ident>()?;
                let keyword_str = keyword.to_string();
                let new_repr = match keyword_str.as_str() {
                    "C" => $enum_ty::C,
                    "transparent" => $enum_ty::Transparent,
                    "packed" => {
                        ret.packed = Some(if input.peek(syn::token::Paren) {
                        let contents; parenthesized!(contents in input);
                        ::syn::LitInt::base10_parse::<u32>(&contents.parse()?)?
                        } else {
                            1
                        });
                        let _: ::core::option::Option<Token![,]> = input.parse()?;
                        continue;
                    },
                    "align" => {
                        let contents; parenthesized!(contents in input);
                        ret.align = Some(syn::LitInt::base10_parse::<u32>(&contents.parse()?)?);
                        let _: ::core::option::Option<Token![,]> = input.parse()?;
                        continue;
                    },
                    $(
                        stringify!($inner) => $enum_ty::$outer,
                    )*
                    _ => return Err(input.error("unrecognized representation hint"))
                };
                if ::core::mem::replace(&mut ret.repr, new_repr) != $enum_ty::Rust {
                    input.error("duplicate representation hint");
                }
                let _: ::core::option::Option<Token![,]> = input.parse()?;
            }
          Ok(ret)
        }
      }

        impl ::quote::ToTokens for $parent {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let repr = match self.repr {
                    $enum_ty::Rust => None,
                    $enum_ty::C => Some(quote!(C)),
                    $enum_ty::Transparent => Some(quote!(transparent)),
                    $(
                        $enum_ty::$outer => Some(quote!($inner)),
                    )*
                };

                let packed = self.packed.map(|p| {
                    let lit = ::syn::LitInt::new(&p.to_string(), Span::call_site());
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
    }
}

impl_comptime_layout! {
    @ComptimeLayout with Repr:
    U8 => u8,
    U16 => u16,
    U32 => u32,
    U64 => u64,
    U128 => u128,
    Usize => usize,
    I8 => i8,
    I16 => i16,
    I32 => i32,
    I64 => i64,
    I128 => i128,
    Isize => isize,
}

pub struct Attribute {
    ident: String,
}

pub enum AttributeKind {
    /// Outer elements or items comprising an attribute.
    ///
    /// The "repr" in the following source code: `#[repr(C, packed(2))]`
    Outer,
    /// Inner elements or items comprising an attribute.
    ///
    /// The "repr" in the following source code: `#[repr(C, packed(2))]`
    Inner(Meta),
}

pub fn parse_attr_repr(attr: &syn::Attribute, ident: &str) -> syn::Result<ComptimeLayout> {
    if attr.path().is_ident(ident) {
        attr.parse_args::<ComptimeLayout>()
    } else {
        Err(Error::new_spanned(attr, "Failed while attempting to parse attribute for layout"))
    }
}
