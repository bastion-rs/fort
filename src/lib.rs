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
pub fn root(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

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
