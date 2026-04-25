extern crate proc_macro;

use darling::{FromDeriveInput, FromMeta, util::Flag};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[derive(Debug, FromMeta)]
#[allow(dead_code)]
pub(crate) struct StructAttr {
    /// The hasher identifier to wrap as an FStr
    hasher: Option<Ident>,
    /// Use a hasher's IO traits directly
    no_io_wrapper: Flag,
    /// Manually set the name of the hash instead of using the hasher ident
    hash_name: Option<String>,
    /// Use a different digest trait
    digest: Option<Ident>,
    /// Hash length constant
    con: Option<Ident>,
}

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

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct MacroArgs {
    hasher: Ident,
    con: Ident,
}

#[proc_macro]
/// Generates impls for the simplest of hashers that don't require anything special
pub fn impl_hash_string(input: TokenStream) -> TokenStream {
    let args: MacroArgs = match syn::parse(input) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let hasher = args.hasher;
    let con = args.con;
    let struct_lower = format!("{}{}", hasher, "String");
    let struct_upper = format!("{}{}", hasher, "StringUpper");
    let ident_lower = syn::Ident::new(&struct_lower, proc_macro2::Span::call_site());
    let ident_upper = syn::Ident::new(&struct_upper, proc_macro2::Span::call_site());

    let expanded = quote! {
        #[derive(Debug, Clone, Eq, hash_strings_derive::StringDigest, hash_strings_derive::StringWrapper)]
        #[hash_strings(hasher = #hasher, con = #con)]
        pub struct #ident_lower(pub fstr::FStr<{
            use digest::typenum::Unsigned;
            #con::USIZE * 2
        }>);
        hash_strings_derive::impl_hash_string_tests!(hasher = #hasher, con = #con);

        #[derive(Debug, Clone, Eq, hash_strings_derive::StringDigest, hash_strings_derive::StringWrapper)]
        #[hash_strings(hasher = #hasher, con = #con)]
        pub struct #ident_upper(pub fstr::FStr<{
            use digest::typenum::Unsigned;
            #con::USIZE * 2
        }>);
    };

    expanded.into()
}
