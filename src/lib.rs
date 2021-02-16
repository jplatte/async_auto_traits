//! Assert and mask auto traits in `async fn` return types.
//!
//! WIP. More docs to come, and I'll look into whether this can be used for
//! `async` blocks too.

use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Token};

mod assert;
mod set;

use assert::expand_assert;
use set::expand_set;

type AutoTraitList = Punctuated<Ident, Token![+]>;

/// Assert that the future returned by an `async fn` implements the given auto traits.
///
/// # Example
///
/// ```rust
/// #[async_auto_traits::assert(Send + Sync)]
/// async fn foo() {}
/// ```
#[proc_macro_attribute]
pub fn assert(attr: TokenStream, item: TokenStream) -> TokenStream {
    let traits = parse_macro_input!(attr with AutoTraitList::parse_separated_nonempty);
    let item = parse_macro_input!(item as ItemFn);
    check_asyncness(&item);
    expand_assert(item, traits).into()
}

/// Clear all auto traits from the `async fn`s return type.
///
/// This turns the `async fn` into a regular `fn` with an `impl Future` return type.
/// Use this when you don't want to promise too much about the future returned by this function.
///
/// # Example
///
/// ```rust
/// #[async_auto_traits::clear]
/// async fn foo() {}
/// ```
#[proc_macro_attribute]
pub fn clear(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("this attribute does not have any parameters.");
    }

    let item = parse_macro_input!(item as ItemFn);
    check_asyncness(&item);
    expand_set(item, None).into()
}

/// Set auto traits for this `async fn`s return type.
///
/// This turns the `async fn` into a regular `fn` with an `impl Future + X + Y` return type.
/// It also serves as an assertion that those auto traits are valid for the `async fn`s body, i.e.
/// this does not invoke any `unsafe` code and using [`macro@assert`] additionally is redundant.
/// Use this when you don't want to promise too much about the future returned by this function.
///
/// # Example
///
/// ```rust
/// #[async_auto_traits::set(Send)]
/// async fn foo() {}
/// ```
#[proc_macro_attribute]
pub fn set(attr: TokenStream, item: TokenStream) -> TokenStream {
    let traits = parse_macro_input!(attr with AutoTraitList::parse_separated_nonempty);
    let item = parse_macro_input!(item as ItemFn);
    check_asyncness(&item);
    expand_set(item, Some(traits)).into()
}

fn check_asyncness(item: &ItemFn) {
    if item.sig.asyncness.is_none() {
        panic!("this attribute only works on `async fn`");
    }
}
