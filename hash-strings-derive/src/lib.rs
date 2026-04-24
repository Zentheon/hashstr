extern crate proc_macro;

use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[derive(Debug, FromMeta)]
#[allow(dead_code)]
pub(crate) struct StructAttr {
    hasher: Option<Ident>,
    hash_name: Option<String>,
    digest: Option<Ident>,
    con: Option<Ident>,
}

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
    let struct_name = format!("{}{}", hasher, "String");
    let struct_ident = syn::Ident::new(&struct_name, proc_macro2::Span::call_site());

    let expanded = quote! {
        #[derive(Debug, Clone, hash_strings_derive::StringDigest, hash_strings_derive::StringWrapper)]
        #[hash_string(hasher = #hasher, con = #con)]
        pub struct #struct_ident(pub fstr::FStr<{
            use digest::typenum::Unsigned;
            #con::USIZE * 2
        }>);
    };

    expanded.into()
}
