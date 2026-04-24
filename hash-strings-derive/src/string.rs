use darling::FromDeriveInput;
use proc_macro_error::abort;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::StructAttr;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(hash_string),
    supports(struct_any),
    // forward_attrs(allow, doc, cfg)
)]
pub struct StringWrapperRec {
    ident: syn::Ident,
    // generics: syn::Generics,
    // data: ast::Data<(), FieldRec>,
    #[darling(flatten)]
    attr: StructAttr,
}

impl ToTokens for StringWrapperRec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StringWrapperRec {
            ref ident,
            // ref generics,
            // ref data,
            ref attr,
        } = *self;

        let ident_str = ident.to_string();
        let hasher = attr.hasher.clone();
        let hash_name = if let Some(name) = &attr.hash_name {
            name.to_string()
        } else if let Some(name) = hasher {
            name.to_string()
        } else {
            abort!(
                attr.hash_name,
                "Either hasher or hash_name attribute must be set"
            )
        };
        let con = attr.con.clone();

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
                type Error = hash_strings::Error;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    Self::try_from(value.to_string())
                }
            }

            impl TryFrom<String> for #ident {
                type Error = hash_strings::Error;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    Self::check_hex(&value)?;
                    Ok(Self(value.parse()?))
                }
            }

            impl std::str::FromStr for #ident {
                type Err = hash_strings::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::try_from(s)
                }
            }

            impl From<&digest::array::Array<u8, #con>> for #ident {
                fn from(digest: &digest::array::Array<u8, #con>) -> Self {
                    Self(base16ct::lower::encode_string(digest).try_into().unwrap())
                }
            }

            impl From<digest::array::Array<u8, #con>> for #ident {
                fn from(digest: digest::array::Array<u8, #con>) -> Self {
                    Self::from(&digest)
                }
            }

            impl PartialEq<#ident> for String {
                fn eq(&self, other: &#ident) -> bool {
                    *self == *other.0
                }
            }

            impl PartialEq<#ident> for str {
                fn eq(&self, other: &#ident) -> bool {
                    *self == *other.0
                }
            }

            impl<O: AsRef<str>> PartialEq<O> for #ident {
                fn eq(&self, other: &O) -> bool {
                    &self.0 == other.as_ref()
                }
            }

            impl Eq for #ident {}

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

            impl serde::Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(&self.0)
                }
            }

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
                fn check_hex(value: &str) -> Result<(), hash_strings::Error> {
                    use hash_strings::{Error, EncodingError};

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
