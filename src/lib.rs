//!
//! Proc macro attributes for Bastion runtime.
//!
//! For more information visit [Bastion](https://docs.rs/bastion) project documentation.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{NestedMeta, AttributeArgs, Lit, LitInt, Meta, MetaNameValue};

/// Supplies bastion runtime to given `main`
///
/// # Examples
///
/// ```ignore
/// #[fort::root]
/// fn main() {
///     println!("Running in Bastion runtime!");
/// }
/// ```
#[cfg(not(test))]
#[proc_macro_attribute]
pub fn root(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let attr_args = syn::parse_macro_input!(attr as syn::AttributeArgs);

    let ret = &input.sig.output;
    let args = input.sig.inputs.iter();
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    if name != "main" {
        let tokens = quote_spanned! { name.span() =>
          compile_error!("only the main function can be tagged with #[fort::root]");
        };
        return TokenStream::from(tokens);
    }

    let retry_time = if let Some(retry_meta) = get_meta(&attr_args, "retry") {
        parse_retry(&retry_meta)
    } else {
        0
    };

    let result = quote! {
        use bastion::prelude::*;

        fn main() #ret {
            #(#attrs)*
            fn main(#(#args),*) #ret {
                #body
            }

            Bastion::platform();
            Bastion::spawn(|context: BastionContext, msg: Box<dyn Message>| {
                    main();
                    context.hook();
                },
                "",
            );
            Bastion::start()
        }
    };

    result.into()
}

fn parse_retry(name_value: &MetaNameValue) -> usize {
    if let Lit::Int(lit_int) = &name_value.lit {
        lit_int.base10_parse::<usize>().unwrap()
    } else {
        0
    }
}

fn get_meta(attr_args: &AttributeArgs, ident: &str) -> Option<MetaNameValue> {
    let mut result = None;
    for attr_args in attr_args {
        if let NestedMeta::Meta(Meta::NameValue(name_value)) = attr_args {
            if name_value.path.is_ident(ident) {
                result = Some(name_value.clone())
            }
        }
    }

    result
}
