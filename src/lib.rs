#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Lit, Meta, NestedMeta, ReturnType};

#[proc_macro_attribute]
pub fn root(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let args = syn::parse_macro_input!(attr as syn::AttributeArgs);

    let body = &input.block;
    let attrs = &input.attrs;
    let inputs = &input.sig.inputs;
    let ret = &input.sig.output;

    if input.sig.asyncness.is_none() {
        let msg = "functions tagged with '#[fort::root]' should be declared as 'async'";
        return Error::new_spanned(input.sig.fn_token, msg)
            .to_compile_error()
            .into();
    } else if input.sig.ident != "main" {
        let msg = "only the main function can be tagged with '#[fort::root]'";
        return Error::new_spanned(input.sig.ident, msg)
            .to_compile_error()
            .into();
    } else if inputs.len() != 1 {
        let msg = "functions tagged with '#[fort::root]' should have one argument of type 'BastionContext'";
        return Error::new_spanned(inputs, msg).to_compile_error().into();
    } else if let ReturnType::Default = ret {
        let msg = "functions tagged with '#[fort::root]' should return 'Result<(), ()>'";
        return Error::new_spanned(ret, msg).to_compile_error().into();
    }

    // TODO: assert!( inputs == [`::bastion::BastionContext`] );
    // TODO: assert!( ret == `::std::result::Result<(), ()>` );

    let mut redundancy = 1;
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(meta)) = arg {
            let ident = meta.path.get_ident();
            if ident.is_none() {
                let msg = "must specify an ident";
                return Error::new_spanned(meta, msg).to_compile_error().into();
            }

            let ident = ident.unwrap();
            match ident.to_string().as_str() {
                "redundancy" => {
                    if let Lit::Int(n) = meta.lit {
                        redundancy = n.base10_parse::<usize>().unwrap();
                    } else {
                        let msg = "'redundancy' should be a number";
                        return Error::new_spanned(meta.lit, msg).to_compile_error().into();
                    }
                }
                _ => {
                    let msg = "unknown attribute";
                    return Error::new_spanned(ident, msg).to_compile_error().into();
                }
            }
        }
    }

    (quote! {
        fn main() {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            bastion::Bastion::init();
            bastion::Bastion::children(|children| {
                children
                    .with_exec(|ctx| main(ctx))
                    .with_redundancy(#redundancy)
            }).expect("Couldn't create the main children group.");

            bastion::Bastion::start();
            bastion::Bastion::block_until_stopped();
        }
    })
    .into()
}
