extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod hashing;

#[proc_macro_derive(StringDigest, attributes(hash_string))]
pub fn derive_string_digest(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    let receiver = hashing::StringDigestRec::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}

mod string;

#[proc_macro_derive(StringWrapper, attributes(hash_string))]
pub fn derive_string_wrapper(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    let receiver = string::StringWrapperRec::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}
