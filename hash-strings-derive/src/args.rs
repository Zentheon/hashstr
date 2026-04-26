use darling::{FromMeta, util::Flag};
use proc_macro_error::abort;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, LitStr};

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
#[allow(dead_code)]
pub(crate) struct Args {
    /// The hasher identifier to wrap as an FStr
    pub hasher: Option<Ident>,
    /// Use a hasher's IO traits directly
    pub no_io_wrapper: Flag,
    /// Manually set the name of the hash instead of using the hasher ident
    pub hash_name: Option<String>,
    /// Use a different digest trait
    pub digest: Option<Ident>,
    /// Hash length constant
    pub con: Option<Ident>,
    /// List of tests to ignore implementing via `impl_hash_string_tests`
    #[darling(default)]
    pub ignore_tests: Vec<LitStr>,
}

impl Args {
    pub fn unwrap_hasher(&self) -> Ident {
        if let Some(hasher) = &self.hasher {
            hasher.clone()
        } else {
            abort!(self.hasher, "hasher attribute must be set")
        }
    }
    pub fn unwrap_hash_name(&self) -> String {
        if let Some(name) = &self.hash_name {
            name.to_string()
        } else if let Some(name) = self.hasher.clone() {
            name.to_string()
        } else {
            abort!(
                self.hash_name,
                "Either hasher or hash_name attribute must be set"
            )
        }
    }
    pub fn hash_name_str(&self) -> LitStr {
        if let Some(name) = &self.hash_name {
            LitStr::new(name, Span::call_site())
        } else if let Some(name) = self.hasher.clone() {
            LitStr::new(&name.to_string(), Span::call_site())
        } else {
            abort!(
                self.hash_name,
                "Either hasher or hash_name attribute must be set"
            )
        }
    }
    pub fn unwrap_digest(&self) -> proc_macro2::TokenStream {
        match self.digest.clone() {
            Some(idt) => quote! { #idt },
            None => quote! { digest::Digest },
        }
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
    pub fn unwrap_ident_lower(&self) -> Ident {
        let name = format!("{}String", self.unwrap_hash_name());
        syn::Ident::new(&name, proc_macro2::Span::call_site())
    }
    pub fn unwrap_ident_upper(&self) -> Ident {
        let name = format!("{}StringUpper", self.unwrap_hash_name());
        syn::Ident::new(&name, proc_macro2::Span::call_site())
    }
}
