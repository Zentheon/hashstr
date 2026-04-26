use proc_macro::TokenStream;
use quote::quote;

use crate::Args;

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
        pub struct #ident_lower(fstr::FStr<{
            use digest::typenum::Unsigned;
            #con::USIZE * 2
        }>);

        #[derive(Debug, Clone, Eq, hash_strings_derive::StringDigest, hash_strings_derive::StringWrapper)]
        #[hash_strings(hasher = #hasher, con = #con)]
        pub struct #ident_upper(fstr::FStr<{
            use digest::typenum::Unsigned;
            #con::USIZE * 2
        }>);

        hash_strings_derive::impl_hash_string_tests!(hasher = #hasher, con = #con);
    };

    expanded.into()
}
