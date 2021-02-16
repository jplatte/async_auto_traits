use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemFn, ReturnType};

use crate::AutoTraitList;

pub fn expand_set(item: ItemFn, traits: Option<AutoTraitList>) -> TokenStream {
    let ItemFn { attrs, vis, mut sig, block } = item;

    let fut_output = match sig.output {
        ReturnType::Default => parse_quote! { () },
        ReturnType::Type(_, ty) => ty,
    };

    sig.asyncness = None;
    sig.output = parse_quote! {
        impl Future<Output = #fut_output $(+ #traits)*>
    };

    quote! {
        #(#attrs)*
        #vis #sig {
            async move #block
        }
    }
}
