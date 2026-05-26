use std::str::FromStr;

use darling::FromMeta;
use proc_macro_error::abort;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, LitStr, Path};

use crate::ident;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, FromMeta)]
pub(crate) enum EncodingType {
    #[default]
    LowerHex,
    UpperHex,
    Base64,
}

impl ToTokens for EncodingType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let extend = match self {
            EncodingType::LowerHex => quote!(LowerHex),
            EncodingType::UpperHex => quote!(UpperHex),
            EncodingType::Base64 => quote!(Base64),
        };
        tokens.extend(extend)
    }
}

impl EncodingType {
    pub fn iterator() -> impl Iterator<Item = EncodingType> {
        [EncodingType::LowerHex, EncodingType::UpperHex, EncodingType::Base64]
            .iter()
            .copied()
    }
}

/// Contains pretty much all the info needed for the macros to work their magic.
#[derive(Debug, Clone, FromMeta)]
#[darling(derive_syn_parse)]
#[allow(dead_code)]
pub(crate) struct Args {
    /// The hasher identifier to wrap.
    pub hasher: Option<Path>,
    /// Name of the crate the hasher belongs to. The module that implements this hashstr should be
    /// named the same.
    pub hasher_crate: Option<Ident>,
    /// The type of encoding of the generated hashstr.
    #[darling(default)]
    pub encoding: EncodingType,
    /// Manually set the name of the hash instead of using the hasher ident
    pub hash_name: Option<String>,
    /// Hash length constant
    pub con: Option<Ident>,
    /// The code in the `digest` method that creates the `hash` variable that is an instance of
    /// Self
    #[darling(default)]
    pub digest: Option<String>,
    /// The code in the `digest_reader` method that creates the `hash` variable that is an instance
    /// of Self
    #[darling(default)]
    pub digest_reader: Option<String>,
    /// The code in the `digest_file` method that creates the `hash` variable that is an instance
    /// of Self
    #[darling(default)]
    pub digest_file: Option<String>,
    /// List of tests to ignore implementing via `impl_hashstr_tests`
    #[darling(default)]
    pub ignore_tests: Vec<LitStr>,
}

impl Args {
    /// Default impl for any given `digest` method
    pub fn digest(&self) -> TokenStream {
        let hasher = self.hasher_ident();
        match &self.digest {
            Some(expr) => TokenStream::from_str(expr).unwrap(),
            None => quote! {
                use digest::Digest;

                let hash =#hasher::digest(data).into();
            },
        }
    }
    /// Default impl for any given `digest_reader` method
    pub fn digest_reader(&self) -> TokenStream {
        let hasher = self.hasher_ident();
        match &self.digest_reader {
            Some(expr) => TokenStream::from_str(expr).unwrap(),
            None => quote! {
                use digest::Digest;

                let mut hasher = digest_io::IoWrapper(#hasher::new());
                let digested = std::io::copy(read, &mut hasher)?;
                let hash = hasher.0.finalize().into();
            },
        }
    }
    /// Default impl for any given `digest_file` method
    pub fn digest_file(&self) -> TokenStream {
        let hasher = self.hasher_ident();
        match &self.digest_file {
            Some(expr) => TokenStream::from_str(expr).unwrap(),
            None => quote! {
                use digest::Digest;

                let mut file = std::fs::File::open(path.as_ref())?;
                let mut hasher = digest_io::IoWrapper(#hasher::new());
                let digested = std::io::copy(&mut file, &mut hasher)?;
                let hash = hasher.0.finalize().into();
            },
        }
    }
    /// Ident of the underlying hasher.
    pub fn hasher_ident(&self) -> Ident {
        if let Some(path) = &self.hasher {
            if let Some(seg) = path.segments.last() {
                seg.ident.clone()
            } else {
                abort!(
                    self.hasher,
                    "hasher attribute does not contain a valid path"
                )
            }
        } else {
            abort!(self.hasher, "hasher attribute must be set")
        }
    }
    /// String literal token of the hasher for use in docs.
    pub fn hash_name_str(&self) -> LitStr {
        if let Some(ident) = &self.hash_name {
            LitStr::new(ident, Span::call_site())
        } else if self.hasher.is_some() {
            LitStr::new(&self.hasher_ident().to_string(), Span::call_site())
        } else {
            abort!(
                self.hash_name,
                "Either hasher or hash_name attribute must be set"
            )
        }
    }
    /// Get the inner ident of this hashstr's typenum const.
    pub fn unwrap_con(&self) -> Ident {
        if let Some(con) = &self.con {
            con.clone()
        } else {
            abort!(self.con, "Hash length constant is required")
        }
    }
    /// Expression that results in the usize of a typenum const
    pub fn unwrap_con_usize(&self) -> proc_macro2::TokenStream {
        let con = self.unwrap_con();
        quote! {
            {
                use digest::typenum::Unsigned;
                #con::USIZE
            }
        }
    }
    /// Expression that results in the usize of a typenum const * 2
    pub fn unwrap_con_usize_x2(&self) -> proc_macro2::TokenStream {
        let con = self.unwrap_con();
        quote! {
            {
                use digest::typenum::Unsigned;
                #con::USIZE * 2
            }
        }
    }
    /// The struct ident. Joins [`Self::hash_name`] with a suffix based on encoding.
    pub fn struct_ident(&self, encoding: EncodingType) -> Ident {
        let base = self.hash_name_str().value();
        match encoding {
            EncodingType::LowerHex => ident!("{base}Hex"),
            EncodingType::UpperHex => ident!("{base}HexUpper"),
            EncodingType::Base64 => ident!("{base}Base64"),
        }
    }
    /// The struct ident in snake case.
    pub fn struct_snake(&self, encoding: EncodingType) -> Ident {
        let base = self.hash_name_str().value().to_lowercase();
        match encoding {
            EncodingType::LowerHex => ident!("{base}_hex_upper"),
            EncodingType::UpperHex => ident!("{base}_hex"),
            EncodingType::Base64 => ident!("{base}_base64"),
        }
    }
    /// Relative path inside the crate to the struct. In other words, without `crate`/`hashstr`
    /// root.
    ///
    /// Used in generated documentation.
    pub fn struct_path(&self, encoding: EncodingType) -> Path {
        let ident = self.struct_ident(encoding);
        let mod_name = self.hasher.as_ref().unwrap().segments[0].ident.clone();
        Path::from_string(format!("{mod_name}::{ident}").as_str()).unwrap()
    }
    /// String literal "upper" or "lower" based on [`Self::encoding`]
    pub fn casing(&self) -> &str {
        if self.encoding == EncodingType::UpperHex {
            "upper"
        } else {
            "lower"
        }
    }
    /// `true` if [`Self::EncodingType`] == [`EncodingType::UpperHex`]
    pub fn is_upper(&self) -> bool {
        match self.encoding == EncodingType::UpperHex {
            true => true,
            false => false,
        }
    }
    /// `true` if [`Self::EncodingType`] == [`EncodingType::LowerHex`] or [`EncodingType::UpperHex`]
    pub fn is_hex(&self) -> bool {
        match self.encoding == EncodingType::LowerHex || self.encoding == EncodingType::UpperHex {
            true => true,
            false => false,
        }
    }
    /// `true` if [`Self::EncodingType`] == [`EncodingType::Base64`]
    pub fn is_base64(&self) -> bool {
        match self.encoding == EncodingType::Base64 {
            true => true,
            false => false,
        }
    }
}
