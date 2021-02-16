use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

use crate::AutoTraitList;

pub fn expand_assert(item: ItemFn, traits: AutoTraitList) -> TokenStream {
    let ItemFn { attrs, vis, sig, block } = item;

    quote! {
        #(#attrs)*
        #vis #sig {
            fn _assert_traits<T: #traits>(val: T) -> T { val }

            let res = async move #block;
            _assert_traits(res).await
        }
    }
}
