use std::str::FromStr;

use darling::FromMeta;
use proc_macro_error::abort;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitBool, LitStr, Path};

use crate::ident;

#[derive(Debug, Clone, FromMeta)]
#[darling(derive_syn_parse)]
#[allow(dead_code)]
pub(crate) struct Args {
    /// The hasher identifier to wrap.
    pub hasher: Option<Path>,
    /// Name of the crate the hasher belongs to. The module that implements this hash-str should be
    /// named the same.
    pub hasher_crate: Option<Ident>,
    /// If the encoded hex should be enforced as uppercase.
    #[darling(default)]
    pub upper: bool,
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
    /// List of tests to ignore implementing via `impl_hash_string_tests`
    #[darling(default)]
    pub ignore_tests: Vec<LitStr>,
}

impl Args {
    pub fn unwrap_hasher(&self) -> Path {
        if let Some(hasher) = &self.hasher {
            hasher.clone()
        } else {
            abort!(self.hasher, "hasher attribute must be set")
        }
    }
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
    pub fn unwrap_hasher_crate(&self) -> Ident {
        if let Some(hasher_crate) = &self.hasher_crate {
            hasher_crate.clone()
        } else {
            abort!(self.hasher_crate, "hasher_crate attribute must be set")
        }
    }
    pub fn hash_name(&self) -> String {
        if let Some(name) = &self.hash_name {
            name.to_string()
        } else {
            self.hasher_ident().to_string()
        }
    }
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
    pub fn hasher_crate_str(&self) -> LitStr {
        LitStr::new(&self.unwrap_hasher_crate().to_string(), Span::call_site())
    }
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
    pub fn struct_ident(&self, upper: bool) -> Ident {
        let base = self
            .hash_name
            .clone()
            .unwrap_or(self.hasher_ident().to_string());
        if upper {
            ident!("{base}StringUpper")
        } else {
            ident!("{base}String")
        }
    }
    pub fn struct_path(&self, upper: bool) -> Path {
        let ident = self.struct_ident(upper);
        let mod_name = self.hasher.as_ref().unwrap().segments[0].ident.clone();
        Path::from_string(format!("{mod_name}::{ident}").as_str()).unwrap()
    }
    pub fn struct_snake(&self, upper: bool) -> Ident {
        let base = self
            .hash_name
            .clone()
            .unwrap_or(self.hasher_ident().to_string())
            .to_lowercase();
        if upper {
            ident!("{base}_string_upper")
        } else {
            ident!("{base}_string")
        }
    }
    pub fn casing(&self) -> &str {
        if self.upper { "upper" } else { "lower" }
    }
    pub fn is_upper(&self) -> LitBool {
        LitBool::new(self.upper, proc_macro2::Span::call_site())
    }
}
