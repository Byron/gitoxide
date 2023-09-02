//! A crate of
use proc_macro::TokenStream;

/// Generate lightweight monomorphized wrapper around main implementation.
/// May be applied to functions and methods.
#[proc_macro_attribute]
pub fn momo(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    //TODO: alternatively parse ImplItem::Method
    momo::inner(input.into()).into()
}

mod momo;
