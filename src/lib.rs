//! Assert and mask auto traits in `async fn` return types.
//!
//! WIP. More docs to come, and I'll look into whether this can be used for
//! `async` blocks too.

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

mod auto_trait_list;
mod set;

use auto_trait_list::AutoTraitList;
use set::expand_set;

/// Set auto traits for this `async fn`s return type.
///
/// Only `Send` and `Sync` are currently supported.
///
/// This adds an assertion that those auto traits are valid for the `async fn`s body, i.e. it does
/// not invoke any `unsafe` code and will produce a compile error when the auto traits are not
/// satisfied.
///
/// # Example
///
/// ```rust
/// #[async_auto_traits::assert(Send + Sync)]
/// async fn foo() {}
/// ```
///
/// # Planned features
///
/// Writing `!Trait` should allow you to explicitly opt out of `Send` or `Sync` if it would
/// otherwise be inferred. This is for when you don't want to promise too much about the future
/// returned by a function to avoid future breaking changes.
///
/// ## Example
///
/// ```rust
/// #[async_auto_traits::assert(Send + !Sync)]
/// async fn foo() {}
/// ```
#[proc_macro_attribute]
pub fn set(attr: TokenStream, item: TokenStream) -> TokenStream {
    let traits = parse_macro_input!(attr as AutoTraitList);
    let item = parse_macro_input!(item as ItemFn);

    if item.sig.asyncness.is_none() {
        panic!("this attribute only works on `async fn`");
    }

    expand_set(item, traits).unwrap_or_else(syn::Error::into_compile_error).into()
}
