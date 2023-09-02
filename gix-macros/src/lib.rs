//! A crate of useful macros used in `gix` primarily.
//!
//! Note that within `gix-*` crates, monomorphization should never be used for convenience, but only for performance
//! reasons. And in the latter case, manual denomophization should be considered if the trait in questions isn't called
//! often enough or measurements indicate that `&dyn Trait` is increasing the runtime. Thus, `gix-*` crates should probably
//! by default prefer using `&dyn` unless measurements indicate otherwise.
use proc_macro::TokenStream;

/// When applied to functions or methods, it will turn it into a wrapper that will immediately call
/// a de-monomorphized implementation (i.e. one that uses `&dyn Trait`).
///
/// That way, the landing-pads for convenience will be as small as possible which then delegate to a single
/// function or method for implementation.
///
/// The parameters using the following traits can be de-monomorphized:
///
/// * `Into`
/// * `AsRef`
/// * `AsMut`
#[proc_macro_attribute]
pub fn momo(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    momo::inner(input.into()).into()
}

mod momo;
