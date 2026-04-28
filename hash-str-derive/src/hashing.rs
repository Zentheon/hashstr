use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::Args;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(hash_str),
    supports(struct_any),
    // forward_attrs(allow, doc, cfg)
)]
pub struct HashStrDigestRec {
    ident: syn::Ident,
    // generics: syn::Generics,
    // data: ast::Data<(), FieldRec>,
    #[darling(flatten)]
    attr: Args,
}
impl ToTokens for HashStrDigestRec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let HashStrDigestRec {
            ref ident,
            // ref generics,
            // ref data,
            ref attr,
        } = *self;
        tokens.extend(Self::generate(ident, attr))
    }
}

impl HashStrDigestRec {
    pub fn generate(ident: &Ident, args: &Args) -> TokenStream {
        let mut tokens = TokenStream::new();

        let digest = args.digest();
        let digest_reader = args.digest_reader();
        let digest_file = args.digest_file();

        // let (imp, ty, wher) = generics.split_for_impl();

        tokens.extend(quote! {
            impl Default for #ident {
                fn default() -> Self {
                    Self::digest([])
                }
            }
            impl #ident {
                /// Digest some raw data and produce a hash.
                pub fn digest(data: impl AsRef<[u8]>) -> Self {
                    #digest

                    #[cfg(feature = "tracing")]
                    tracing::trace!(%hash, "Generated a hash of raw data");
                    hash
                }
                /// Attempts to digest the entirety of the given reader.
                ///
                /// Returns errors produced by [`std::io::copy`]
                pub fn digest_reader<R>(read: &mut R) -> Result<Self, std::io::Error>
                where
                    R: Sized,
                    R: std::io::Read,
                {
                    #digest_reader

                    #[cfg(feature = "tracing")]
                    tracing::trace!(digested, %hash, "Generated the hash of content in a reader");

                    Ok(hash)
                }
                /// Attempts to open the given path and digest the entirety of its bytes.
                ///
                /// Returns errors produced by [`std::fs::File::open`] and [`std::io::copy`]
                pub fn digest_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
                    #digest_file

                    #[cfg(feature = "tracing")]
                    tracing::trace!(digested, %hash, path = ?path.as_ref(), "Generated the hash of a file");

                    Ok(hash)
                }
            }
        });
        tokens
    }
}
