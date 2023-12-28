extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro]
pub fn is_even(input: TokenStream) -> TokenStream {
    let limit = syn::parse::<syn::LitInt>(input).expect("Expected an integer literal").base10_parse::<usize>().expect("Expected a valid usize");

    let cases = (0..limit).map(|i| {
        if i % 2 == 0 {
            quote! { #i => true, }
        } else {
            quote! { #i => false, }
        }
    });

    let expanded = quote! {
        |x: usize| -> bool {
            match x {
                #(#cases)*
                _ => x % 2 == 0,
            }
        }
    };

    TokenStream::from(expanded)
}

