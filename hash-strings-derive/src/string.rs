use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::Args;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(hash_strings),
    supports(struct_any),
    // forward_attrs(allow, doc, cfg)
)]
pub struct StringWrapperRec {
    ident: syn::Ident,
    // generics: syn::Generics,
    // data: ast::Data<(), FieldRec>,
    #[darling(flatten)]
    attr: Args,
}

impl ToTokens for StringWrapperRec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StringWrapperRec {
            ref ident,
            // ref generics,
            // ref data,
            ref attr,
        } = *self;

        let hash_name = attr.unwrap_hash_name();
        let con = attr.unwrap_con();

        // let (imp, ty, wher) = generics.split_for_impl();

        tokens.extend(quote! {
            impl std::ops::Deref for #ident {
                type Target = fstr::FStr<{
                    use digest::typenum::Unsigned;
                    #con::USIZE * 2
                }>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for #ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl AsRef<[u8]> for #ident {
                fn as_ref(&self) -> &[u8] {
                    self.0.as_bytes()
                }
            }

            impl AsRef<str> for #ident {
                fn as_ref(&self) -> &str {
                    &self.0
                }
            }

            impl std::fmt::Display for #ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", *self)
                }
            }

            impl TryFrom<&str> for #ident {
                type Error = Error;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    Self::check_hex(value)?;
                    let fstr = Error::from_fstr_err(value.parse(), &#hash_name)?;
                    Ok(Self(fstr))
                }
            }

            impl TryFrom<String> for #ident {
                type Error = Error;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    Self::try_from(value.as_str())
                }
            }

            impl std::str::FromStr for #ident {
                type Err = Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::try_from(s)
                }
            }

            impl From<&digest::array::Array<u8, #con>> for #ident {
                fn from(digest: &digest::array::Array<u8, #con>) -> Self {
                    let fstr = fstr::FStr::try_from(base16ct::lower::encode_string(digest)).unwrap();
                    Self(fstr)
                }
            }

            impl From<digest::array::Array<u8, #con>> for #ident {
                fn from(digest: digest::array::Array<u8, #con>) -> Self {
                    Self::from(&digest)
                }
            }

            impl From<&generic_array::GenericArray<u8, #con>> for #ident {
                fn from(digest: &generic_array::GenericArray<u8, #con>) -> Self {
                    Self(base16ct::lower::encode_string(digest).try_into().unwrap())
                }
            }

            impl From<generic_array::GenericArray<u8, #con>> for #ident {
                fn from(digest: generic_array::GenericArray<u8, #con>) -> Self {
                    Self::from(&digest)
                }
            }

            impl PartialEq<#ident> for String {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_ref(), other.0.as_ref())
                }
            }

            impl PartialEq<#ident> for fstr::FStr<{
                use digest::typenum::Unsigned;
                #con::USIZE * 2
            }> {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_ref(), other.0.as_ref())
                }
            }

            impl PartialEq<#ident> for str {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_ref(), other.0.as_ref())
                }
            }

            impl PartialEq<#ident> for &str {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_ref(), other.0.as_ref())
                }
            }

            impl<O: AsRef<[u8]>> PartialEq<O> for #ident {
                fn eq(&self, other: &O) -> bool {
                    constant_time_eq::constant_time_eq(self.0.as_ref(), other.as_ref())
                }
            }

            impl std::hash::Hash for #ident {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.0.hash(state);
                }
            }

            impl PartialOrd for #ident {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.0.cmp(&other.0))
                }
            }

            impl Ord for #ident {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.0.cmp(&other.0)
                }
            }

            #[cfg(feature = "serde")]
            impl serde::Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(&self.0)
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> serde::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let inner_string = String::deserialize(deserializer)?;

                    Self::try_from(inner_string).map_err(|e| serde::de::Error::custom(format!("{e}")))
                }
            }

            impl #ident {
                pub fn as_str(&self) -> &str {
                    &self.0
                }
                fn check_hex(value: &str) -> Result<(), Error> {
                    if !value.chars().all(|c| c.is_digit(16)) {
                        Err(Error::EncodingError(EncodingError { hash_name: #hash_name.to_string() }))
                    } else {
                        Ok(())
                    }
                }
            }
        });
    }
}
