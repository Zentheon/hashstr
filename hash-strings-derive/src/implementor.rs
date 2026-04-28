use proc_macro::TokenStream;
use quote::{TokenStreamExt, quote};

use crate::{Args, hashing::StringDigestRec, string::StringWrapperRec};

/// Generates impls for the simplest of hashers that don't require anything special
pub fn impl_hash_string(input: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(input) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let con = args.unwrap_con();
    let hasher = args.hasher_ident();
    let path = args.hasher.as_ref().unwrap();
    let hash_name = args.hash_name_str();

    let mut expanded = quote!();
    for i in 0..2 {
        let upper = i == 1;
        args.upper = upper;
        let casing = args.casing();
        let ident = args.struct_ident(upper);
        let struct_path = args.struct_path(upper);

        let hashing_impl = StringDigestRec::generate(&ident, &args);
        let wrapper_impl = StringWrapperRec::generate(&ident, &args);

        expanded.append_all(quote! {
            #[doc = concat!("A [`str`]-like, ", #casing, "case hex-encoded representation of a [`", #hash_name, "`] hash.")]
            ///
            /// # Usage
            /// ```rust
            #[doc = concat!("use hash_strings::", stringify!(#struct_path), ";")]
            ///
            /// fn main() -> std::io::Result<()> {
            #[doc = concat!("     let hash1 = ", stringify!(#ident), "::digest(b\"abc\");")]
            ///     println!("Hash of 'abc': {hash1}");
            ///
            ///     let mut reader = std::io::Cursor::new([200 % 44 + (45 ^ 6) - 25]);
            #[doc = concat!("     let hash2 = ", stringify!(#ident), "::digest_reader(&mut reader)?;")]
            ///     println!("Hash of '42': {hash2}");
            ///
            ///     #[cfg(no_run)] // This one is omitted from doctests
            #[doc = concat!("     let hash3 = ", stringify!(#ident), "::digest_file(\"./example.txt\")?;")]
            ///
            ///     assert!(hash1 != hash2);
            ///     Ok(())
            /// }
            /// ```
            /// See crate root for more examples.
            #[derive(Debug, Clone, Eq)]
            pub struct #ident(fstr::FStr<{
                use digest::typenum::Unsigned;
                #con::USIZE * 2
            }>);
            #wrapper_impl
            #hashing_impl

            hash_strings_derive::impl_hash_string_tests!(hasher = #hasher, con = #con, upper = #upper);
        });
    }

    expanded.into()
}
