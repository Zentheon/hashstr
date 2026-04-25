extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod args;
pub(crate) use args::Args;

mod hashing;

#[proc_macro_derive(StringDigest, attributes(hash_strings))]
pub fn derive_string_digest(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    let receiver = hashing::StringDigestRec::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}

mod string;

#[proc_macro_derive(StringWrapper, attributes(hash_strings))]
pub fn derive_string_wrapper(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    let receiver = string::StringWrapperRec::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}

mod tests;

#[proc_macro]
pub fn impl_hash_string_tests(input: TokenStream) -> TokenStream {
    tests::impl_hash_string_tests(input)
}

#[proc_macro]
/// Generates impls for the simplest of hashers that don't require anything special
pub fn impl_hash_string(input: TokenStream) -> TokenStream {
    let args: Args = match syn::parse(input) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let con = args.unwrap_con();
    let hasher = args.unwrap_hasher();
    let ident_lower = args.unwrap_ident_lower();
    let ident_upper = args.unwrap_ident_upper();

    let expanded = quote! {
        #[derive(Debug, Clone, Eq, hash_strings_derive::StringDigest, hash_strings_derive::StringWrapper)]
        #[hash_strings(hasher = #hasher, con = #con)]
        pub struct #ident_lower(pub fstr::FStr<{
            use digest::typenum::Unsigned;
            #con::USIZE * 2
        }>);

        #[derive(Debug, Clone, Eq, hash_strings_derive::StringDigest, hash_strings_derive::StringWrapper)]
        #[hash_strings(hasher = #hasher, con = #con)]
        pub struct #ident_upper(pub fstr::FStr<{
            use digest::typenum::Unsigned;
            #con::USIZE * 2
        }>);

        hash_strings_derive::impl_hash_string_tests!(hasher = #hasher, con = #con);
    };

    expanded.into()
}
