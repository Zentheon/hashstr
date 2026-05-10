use proc_macro::TokenStream;
use quote::{TokenStreamExt, quote};

use crate::{Args, args::EncodingType, hashing::HashStrDigestRec, string::StrWrapperRec};

/// Generates impls for the simplest of hashers that don't require anything special
pub fn impl_hashstr(input: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(input) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let con = args.unwrap_con();
    let hash_name = args.hash_name_str();

    let mut expanded = quote!();
    for encoding in EncodingType::iterator() {
        if let EncodingType::Base64 = encoding {
            continue; // todo
        }

        args.encoding = encoding;
        let ident = args.struct_ident(encoding);
        let struct_path = args.struct_path(encoding);

        let type_doc = match encoding {
            EncodingType::LowerHex => "lowercase hex-encoded",
            EncodingType::UpperHex => "uppercase hex-encoded",
            EncodingType::Base64 => "base64-encoded",
        };

        let hashing_impl = HashStrDigestRec::generate(&ident, &args);
        let wrapper_impl = StrWrapperRec::generate(&ident, &args);
        let tests = crate::generate_tests(&args);

        expanded.append_all(quote! {
            #[doc = concat!("A [`str`]-like, ", #type_doc, "representation of a [`", #hash_name, "`] hash.")]
            ///
            /// # Usage
            /// ```rust
            #[doc = concat!("use hashstr::", stringify!(#struct_path), ";")]
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
            #tests
        });
    }

    expanded.into()
}
