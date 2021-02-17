use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident, ItemFn, ReturnType};

use crate::auto_trait_list::AutoTraitList;

pub fn expand_set(item: ItemFn, traits: AutoTraitList) -> syn::Result<TokenStream> {
    let ItemFn { attrs, vis, mut sig, block } = item;
    let stmts = block.stmts;

    let fut_output = match sig.output {
        ReturnType::Default => parse_quote! { () },
        ReturnType::Type(_, ty) => ty,
    };

    let (regular_tr, opt_out_tr) = traits.partition();
    let opt_outs = get_opt_outs(&opt_out_tr)?;

    sig.asyncness = None;
    sig.output = parse_quote! {
        -> impl ::core::future::Future<Output = #fut_output> #(+ #regular_tr)*
    };

    if opt_outs.send || opt_outs.sync {
        panic!("opt-outs are not yet implemented");
    }

    Ok(quote! {
        #(#attrs)*
        #vis #sig {
            async move {
                #(#stmts)*
            }
        }
    })
}

#[derive(Default)]
struct OptOuts {
    send: bool,
    sync: bool,
}

fn get_opt_outs(traits: &[Ident]) -> syn::Result<OptOuts> {
    let mut opt_outs = OptOuts::default();

    for tr in traits {
        match tr.to_string().as_str() {
            "Send" => opt_outs.send = true,
            "Sync" => opt_outs.sync = true,
            _ => {
                return Err(syn::Error::new_spanned(
                    tr,
                    "only Send and Sync are supported for autotrait opt-out",
                ));
            }
        }
    }

    Ok(opt_outs)
}
