use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as syn::Path);
    let function = parse_macro_input!(item as ItemFn);

    let ident = &function.sig.ident;

    quote! {
        #function

        inventory::submit! {
            #path(#ident)
        }
    }
    .into()
}
