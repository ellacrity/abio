use proc_macro2::TokenStream;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AsBytes;

impl AsBytes {
    pub fn impl_block(input: TokenStream) -> syn::Result<TokenStream> {
        Ok(input)
    }
}
