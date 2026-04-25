use darling::FromDeriveInput;
use proc_macro_error::abort;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::Args;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(hash_strings),
    supports(struct_any),
    // forward_attrs(allow, doc, cfg)
)]
pub struct StringDigestRec {
    ident: syn::Ident,
    // generics: syn::Generics,
    // data: ast::Data<(), FieldRec>,
    #[darling(flatten)]
    attr: Args,
}

impl ToTokens for StringDigestRec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StringDigestRec {
            ref ident,
            // ref generics,
            // ref data,
            ref attr,
        } = *self;

        let hasher = attr
            .hasher
            .clone()
            .unwrap_or_else(|| abort!(attr.hasher, "hasher attribute must be set"));

        let digest = match attr.digest.clone() {
            Some(idt) => quote! { #idt },
            None => quote! { digest::Digest },
        };

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
                    use #digest;

                    let hash = #hasher::digest(data).into();

                    #[cfg(feature = "tracing")]
                    tracing::trace!(%hash, "Generated a hash of raw data");
                    hash
                }
            }
        });

        let (io_init, io_finalize) = if attr.no_io_wrapper.is_present() {
            (quote!(#hasher::new()), quote!(hasher.finalize()))
        } else {
            (
                quote!(digest_io::IoWrapper(#hasher::new())),
                quote!(hasher.0.finalize()),
            )
        };
        tokens.extend(quote! {
            impl #ident {
                /// Attempts to digest the entirety of the given reader.
                ///
                /// Returns errors produced by [`std::io::copy`]
                pub fn digest_reader<R>(read: &mut R) -> Result<Self, std::io::Error>
                where
                    R: Sized,
                    R: std::io::Read,
                {
                    use #digest;

                    let mut hasher = #io_init;
                    let digested = std::io::copy(read, &mut hasher)?;
                    let hash = Self::from(&#io_finalize);

                    #[cfg(feature = "tracing")]
                    tracing::trace!(digested, %hash, "Generated the hash of content in a reader");

                    Ok(hash)
                }
                /// Attempts to open the given path and digest the entirety of its bytes.
                ///
                /// Returns errors produced by [`std::fs::File::open`] and [`std::io::copy`]
                pub fn digest_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
                    use #digest;

                    let mut file = std::fs::File::open(path.as_ref())?;
                    let mut hasher = #io_init;
                    let digested = std::io::copy(&mut file, &mut hasher)?;
                    let hash = Self::from(&#io_finalize);

                    #[cfg(feature = "tracing")]
                    tracing::trace!(digested, %hash, path = ?path.as_ref(), "Generated the hash of a file");

                    Ok(hash)
                }
            }
        });
    }
}
