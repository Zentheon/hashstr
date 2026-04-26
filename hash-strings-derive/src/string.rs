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
        let hash_name_str = attr.hash_name_str();
        let con = attr.unwrap_con();
        let con_int = attr.unwrap_con_usize();
        let con_int_x2 = attr.unwrap_con_usize_x2();

        // let (imp, ty, wher) = generics.split_for_impl();

        tokens.extend(quote! {
            impl std::ops::Deref for #ident {
                type Target = fstr::FStr<#con_int_x2>;

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
                    write!(f, "{}", self.0)
                }
            }

            impl TryFrom<&str> for #ident {
                type Error = Error;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    Self::from_hex(value)
                }
            }

            impl TryFrom<String> for #ident {
                type Error = Error;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    Self::from_hex(value)
                }
            }

            impl std::str::FromStr for #ident {
                type Err = Error;

                fn from_str(value: &str) -> Result<Self, Self::Err> {
                    Self::from_hex(value)
                }
            }

            impl TryFrom<&[u8]> for #ident {
                type Error = Error;

                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    Self::encode_bytes(value)
                }
            }

            impl From<[u8; #con_int]> for #ident {
                fn from(value: [u8; #con_int]) -> Self {
                    Self::encode_bytes(value).unwrap()
                }
            }

            impl From<&hybrid_array::Array<u8, #con>> for #ident {
                fn from(value: &digest::array::Array<u8, #con>) -> Self {
                    Self::encode_bytes(value).unwrap()
                }
            }

            impl From<hybrid_array::Array<u8, #con>> for #ident {
                fn from(value: digest::array::Array<u8, #con>) -> Self {
                    Self::encode_bytes(&value).unwrap()
                }
            }

            impl From<&generic_array::GenericArray<u8, #con>> for #ident {
                fn from(value: &generic_array::GenericArray<u8, #con>) -> Self {
                    Self::encode_bytes(value).unwrap()
                }
            }

            impl From<generic_array::GenericArray<u8, #con>> for #ident {
                fn from(value: generic_array::GenericArray<u8, #con>) -> Self {
                    Self::encode_bytes(value).unwrap()
                }
            }

            impl From<&#ident> for [u8; #con_int] {
                fn from(value: &#ident) -> Self {
                    value.to_array()
                }
            }

            impl<'a> From<&'a #ident> for &'a hybrid_array::Array<u8, #con> {
                fn from(value: &'a #ident) -> Self {
                    value.as_hybrid_array()
                }
            }

            impl<'a> From<&'a #ident> for &'a generic_array::GenericArray<u8, #con> {
                fn from(value: &'a #ident) -> Self {
                    value.as_generic_array()
                }
            }

            impl From<&#ident> for generic_array::GenericArray<u8, #con> {
                fn from(value: &#ident) -> Self {
                    value.to_generic_array()
                }
            }

            impl PartialEq<#ident> for String {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_bytes(), other.as_bytes())
                }
            }

            impl PartialEq<#ident> for fstr::FStr<#con_int_x2> {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_bytes(), other.as_bytes())
                }
            }

            impl PartialEq<#ident> for str {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_bytes(), other.as_bytes())
                }
            }

            impl PartialEq<#ident> for &str {
                fn eq(&self, other: &#ident) -> bool {
                    constant_time_eq::constant_time_eq(self.as_bytes(), other.as_bytes())
                }
            }

            impl<O: AsRef<[u8]>> PartialEq<O> for #ident {
                fn eq(&self, other: &O) -> bool {
                    constant_time_eq::constant_time_eq(self.as_bytes(), other.as_ref())
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
                /// Returns the length of the underlying bytes: `N * 2`
                ///
                /// In other words, the number of hex chars can be derived by deviding the output by 2.
                pub const fn len(&self) -> usize {
                    self.as_bytes().len()
                }
                pub const fn as_str(&self) -> &str {
                    self.0.as_str()
                }
                pub const fn as_bytes(&self) -> &[u8] {
                    self.0.as_bytes()
                }
                pub const fn as_array(&self) -> &[u8; #con_int_x2] {
                    self.0.as_bytes().as_array().unwrap()
                }
                /// Returns a fixed-size array constant.
                pub const fn to_array(&self) -> [u8; #con_int] {
                    use digest::typenum::Unsigned;
                    const N: usize = #con::USIZE;

                    let mut array = [0u8; N];
                    array.copy_from_slice(self.as_bytes());
                    array
                }
                pub const fn as_generic_array(&self) -> &generic_array::GenericArray<u8, #con> {
                    generic_array::GenericArray::from_slice(self.as_bytes())
                }
                pub const fn to_generic_array(&self) -> generic_array::GenericArray<u8, #con> {
                    generic_array::GenericArray::from_array(self.to_array())
                }
                pub const fn as_hybrid_array(&self) -> &hybrid_array::Array<u8, #con> {
                    hybrid_array::Array::slice_as_array(self.as_bytes()).unwrap()
                }
                /// Returns a reference to the underlying [`fstr::FStr`]
                pub const fn as_fstr(&self) -> &fstr::FStr<#con_int_x2>
                {
                    &self.0
                }
                /// Returns a copy of the underlying [`fstr::FStr`]
                pub fn to_fstr(&self) -> fstr::FStr<#con_int_x2>
                {
                    self.0
                }
                /// Encode a hash as a bytes slice into a hash string.
                ///
                /// Most of the [`TryFrom`] impls use this method.
                ///
                /// # Errors
                /// * [`crate::Error::LengthError`]: If the input bytes are not the expected hash length.
                /// * [`crate::Error::EncodingError`]: If any byte is not valid utf-8
                pub fn encode_bytes(value: impl AsRef<[u8]>) -> Result<Self, crate::Error> {
                    let hex = crate::encode_lower_hex::<#con_int_x2>(value, #hash_name_str)?;
                    Ok(Self(hex))
                }
                /// `const` equivalent of [`Self::encode_bytes`].
                ///
                /// If you don't have a super cool compile-time use case, prefer the aforementioned runtime
                /// variant instead.
                ///
                /// # Errors
                /// * [`crate::Error::LengthError`]: If the input bytes are not the expected hash length.
                /// * [`crate::Error::EncodingError`]: If any byte is not valid utf-8
                pub const fn encode_slice(value: &[u8; #con_int_x2]) -> Self {
                    Self(crate::encode_lower_hex_const::<#con_int_x2>(value))
                }
                /// Convert a lowercase hex-encoded str into a hash string.
                ///
                /// Most of the string-related [`TryFrom`] impls use this method.
                pub fn from_hex(value: impl AsRef<str>) -> Result<Self, crate::Error> {
                    let value = value.as_ref();
                    crate::check_len::<#con_int_x2>(value.as_ref(), #hash_name_str)?;
                    const_hex::check(value)
                        .map_err(|e| crate::Error::from_hex_err::<#con_int_x2>(e, value.len(), #hash_name_str))?;

                    // SAFETY: Length and encoding has already been checked above.
                    Ok(Self(unsafe {
                        fstr::FStr::from_inner_unchecked(*value.as_bytes().as_array().unwrap())
                    }))
                }
                /// Convert a hex slice into a hash string.
                ///
                /// # Safety
                /// Input value must, at minimum, be valid UTF-8, and __should__ be lowercase hexadecimal.
                pub const unsafe fn from_hex_unchecked(value: [u8; #con_int_x2]) -> Self {
                    Self(unsafe { fstr::FStr::<#con_int_x2>::from_inner_unchecked(value) })
                }
                /// Returns an uppercase hexadecimal [`String`] of the hash.
                ///
                /// If using an *Upper variant, prefer [`Self::to_string()`] instead.
                pub fn to_uppercase(&self) -> String {
                    self.0.to_uppercase()
                }
                /// Returns a lowercase hexadecimal [`String`] of the hash.
                ///
                /// If using a non *Upper variant, prefer [`Self::to_string()`] instead.
                pub fn to_lowercare(&self) -> String {
                    self.0.to_lowercase()
                }
            }
        });
    }
}
