extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod hashstring;

#[proc_macro_derive(HashString, attributes(hash_string))]
pub fn derive_hash_string(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    let receiver = hashstring::HashStringRec::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}
