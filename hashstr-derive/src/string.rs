use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::{Args, args::EncodingType};

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(hashstr),
    supports(struct_any),
    // forward_attrs(allow, doc, cfg)
)]
pub struct StrWrapperRec {
    ident: syn::Ident,
    // generics: syn::Generics,
    // data: ast::Data<(), FieldRec>,
    #[darling(flatten)]
    attr: Args,
}
impl ToTokens for StrWrapperRec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StrWrapperRec {
            ref ident,
            // ref generics,
            // ref data,
            ref attr,
        } = *self;
        tokens.extend(Self::generate(ident, attr))
    }
}

impl StrWrapperRec {
    pub fn generate(ident: &Ident, args: &Args) -> TokenStream {
        let mut tokens = TokenStream::new();

        let hash_name_str = args.hash_name_str();
        let upper = args.is_upper();
        let casing = args.casing();

        let con = args.unwrap_con();
        let con_int = args.unwrap_con_usize();
        let con_int_x2 = args.unwrap_con_usize_x2();

        // let (imp, ty, wher) = generics.split_for_impl();

        // impls

        tokens.extend(quote! {
            impl #ident {
                /// The number of bytes in this hash.
                ///
                /// Since the encoding is in hex, this always represents the number of characters.
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
                pub const fn to_fstr(&self) -> fstr::FStr<#con_int_x2>
                {
                    self.0
                }
                /// Convert a hex slice into a hash str.
                ///
                /// # Safety
                #[doc = concat!("Input value must, at minimum, be valid UTF-8, and __should__ be ", #casing, "case hexadecimal.")]
                pub const unsafe fn from_inner_unchecked(value: [u8; #con_int_x2]) -> Self {
                    Self(unsafe { fstr::FStr::<#con_int_x2>::from_inner_unchecked(value) })
                }
            }
        });

        // hex-specific

        if args.is_hex() {
            tokens.extend(quote! {
                impl #ident {
                    /// Encode a raw hash into a hash str.
                    ///
                    /// Most of the [`TryFrom`] impls use this method.
                    ///
                    /// # Errors
                    /// * [`crate::Error::LengthError`]: If the input bytes are not the expected (raw) hash length.
                    pub fn encode(value: impl AsRef<[u8]>) -> Result<Self, crate::Error> {
                        let hex = crate::encode_hex::<#con_int_x2>(value, #upper, #hash_name_str)?;
                        Ok(Self(hex))
                    }
                    /// Encode a raw hash array into a hash str.
                    pub fn encode_slice(value: &[u8; #con_int]) -> Self {
                        let hex = crate::encode_hex::<#con_int_x2>(value, #upper, #hash_name_str).unwrap();
                        Self(hex)
                    }
                    #[doc = concat!("Convert a ", #casing, "case hex-encoded str into a hash str.")]
                    ///
                    /// Most of the string-related [`TryFrom`] impls use this method.
                    pub fn from_str(value: impl AsRef<str>) -> Result<Self, crate::Error> {
                        let value = value.as_ref();

                        crate::decode_hex::<#con_int_x2, #upper>(value, #hash_name_str)?;

                        // SAFETY: Length and encoding has already been checked above.
                        Ok(Self(unsafe {
                            fstr::FStr::from_inner_unchecked(
                                crate::convert_hex_case::<#con_int_x2, #upper>(value.as_bytes().as_array().unwrap())
                            )
                        }))
                    }
                    /// Returns an uppercase hexadecimal [`fstr::FStr`] of the hash.
                    ///
                    /// If using an *Upper variant, prefer [`Self::to_fstr`] instead.
                    pub const fn to_uppercase(&self) -> fstr::FStr<#con_int_x2> {
                        crate::convert_hex_case_fstr::<#con_int_x2, true>(&self.0)
                    }
                    /// Returns a lowercase hexadecimal [`fstr::FStr`] of the hash.
                    ///
                    /// If using a non *Upper variant, prefer [`Self::to_fstr`] instead.
                    pub const fn to_lowercase(&self) -> fstr::FStr<#con_int_x2> {
                        crate::convert_hex_case_fstr::<#con_int_x2, false>(&self.0)
                    }
                    /// Returns a raw UTF8 bytes array of the underlying hex converted to the specified casing.
                    pub const fn convert_case_raw<const TO_UPPER: bool>(&self) -> [u8; #con_int_x2] {
                        crate::convert_hex_case::<#con_int_x2, TO_UPPER>(self.as_bytes().as_array().unwrap())
                    }
                }
            });
        }

        // base64-specific

        if args.is_base64() {
            tokens.extend(quote! {
                impl #ident {
                    /// Encodes a raw hash into default, padded base64
                    ///
                    /// See [`base64ct::Base64`] for specifics.
                    ///
                    /// # Errors
                    /// * [`crate::Error::LengthError`]: If the input bytes are not the expected (raw) hash length.
                    fn encode(value: impl AsRef<[u8]>) -> Result<Self, crate::Error> {
                        let bytes = value.as_ref();
                        // Ensure length correctness
                        crate::check_len::<{ base64_encoded_len(64) }>(bytes, #hash_name_str)?;
                        let mut array = [0u8; base64_encoded_len(64)];
                        base64ct::Base64::encode(bytes, &mut array).unwrap();
                        // SAFETY: Output of base64ct encode methods are UTF8 in the ASCII range.
                        Ok(Self(unsafe { fstr::FStr::from_inner_unchecked(array) }))
                    }
                    /// Encodes a raw hash slice into default, padded base64
                    ///
                    /// See [`base64ct::Base64`] for specifics.
                    fn encode_slice(value: &[u8; 64]) -> Self {
                        let mut array = [0u8; base64_encoded_len(64)];
                        base64ct::Base64::encode(value, &mut array).unwrap();
                        // SAFETY: Output of base64ct encode methods are UTF8 in the ASCII range.
                        Self(unsafe { fstr::FStr::from_inner_unchecked(array) })
                    }
                    /// Encodes a raw hash slice into url-safe, padded base64
                    ///
                    /// See [`base64ct::Base64Url`] for specifics.
                    fn encode_slice_url_safe(value: &[u8; 64]) -> Self {
                        let mut array = [0u8; base64_encoded_len(64)];
                        base64ct::Base64Url::encode(value, &mut array).unwrap();
                        // SAFETY: Output of base64ct encode methods are UTF8 in the ASCII range.
                        Self(unsafe { fstr::FStr::from_inner_unchecked(array) })
                    }
                    /// Encodes a raw hash slice into bcrypt, padded base64
                    ///
                    /// See [`base64ct::Base64Bcrypt`] for specifics.
                    fn encode_slice_bcrypt(value: &[u8; 64]) -> Self {
                        let mut array = [0u8; base64_encoded_len(64)];
                        base64ct::Base64Bcrypt::encode(value, &mut array).unwrap();
                        // SAFETY: Output of base64ct encode methods are UTF8 in the ASCII range.
                        Self(unsafe { fstr::FStr::from_inner_unchecked(array) })
                    }
                    /// Encodes a raw hash slice into shacrypt, padded base64
                    ///
                    /// See [`base64ct::Base64ShaCrypt`] for specifics.
                    fn encode_slice_shacrypt(value: &[u8; 64]) -> Self {
                        let mut array = [0u8; base64_encoded_len(64)];
                        base64ct::Base64ShaCrypt::encode(value, &mut array).unwrap();
                        // SAFETY: Output of base64ct encode methods are UTF8 in the ASCII range.
                        Self(unsafe { fstr::FStr::from_inner_unchecked(array) })
                    }
                    #[doc = concat!("Convert a ", #casing, "case hex-encoded str into a hash str.")]
                    ///
                    /// Most of the string-related [`TryFrom`] impls use this method.
                    pub fn from_inner(value: impl AsRef<str>) -> Result<Self, crate::Error> {
                        let value = value.as_ref();
                        crate::check_len::<#con_int_x2>(value.as_ref(), #hash_name_str)?;
                        base16ct::lower::decode(value, &mut decoded)
                            .map_err(|_| crate::Error::EncodingError(EncodingError { #hash_name_str }))?;

                        // SAFETY: Length and encoding has already been checked above.
                        Ok(Self(unsafe {
                            fstr::FStr::from_inner_unchecked(
                                crate::convert_hex_case::<#con_int_x2, #upper>(value.as_bytes().as_array().unwrap())
                            )
                        }))
                    }
                }
            });
        }

        // Misc traits

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
        });

        // From/TryFrom
        //
        // Implementations should be conversions between different types and encodings only. No
        // digesting

        tokens.extend(quote! {
            impl TryFrom<&str> for #ident {
                type Error = crate::Error;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    Self::from_str(value)
                }
            }

            impl TryFrom<String> for #ident {
                type Error = crate::Error;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    Self::from_str(value)
                }
            }

            impl std::str::FromStr for #ident {
                type Err = crate::Error;

                fn from_str(value: &str) -> Result<Self, Self::Err> {
                    Self::from_str(value)
                }
            }

            impl TryFrom<&[u8]> for #ident {
                type Error = crate::Error;

                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    Self::encode(value)
                }
            }

            impl TryFrom<&fstr::FStr<#con_int_x2>> for #ident {
                type Error = crate::Error;

                fn try_from(value: &fstr::FStr<#con_int_x2>) -> Result<Self, Self::Error> {
                    Self::from_str(value)
                }
            }

            impl TryFrom<fstr::FStr<#con_int_x2>> for #ident {
                type Error = crate::Error;

                fn try_from(value: fstr::FStr<#con_int_x2>) -> Result<Self, Self::Error> {
                    Self::from_str(value)
                }
            }

            impl From<[u8; #con_int]> for #ident {
                fn from(value: [u8; #con_int]) -> Self {
                    Self::encode(value).unwrap()
                }
            }

            impl From<&hybrid_array::Array<u8, #con>> for #ident {
                fn from(value: &digest::array::Array<u8, #con>) -> Self {
                    Self::encode(value).unwrap()
                }
            }

            impl From<hybrid_array::Array<u8, #con>> for #ident {
                fn from(value: digest::array::Array<u8, #con>) -> Self {
                    Self::encode(&value).unwrap()
                }
            }

            impl From<&generic_array::GenericArray<u8, #con>> for #ident {
                fn from(value: &generic_array::GenericArray<u8, #con>) -> Self {
                    Self::encode(value).unwrap()
                }
            }

            impl From<generic_array::GenericArray<u8, #con>> for #ident {
                fn from(value: generic_array::GenericArray<u8, #con>) -> Self {
                    Self::encode(value).unwrap()
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
        });

        // Convert between other variants

        if args.encoding == EncodingType::UpperHex {
            let lower_hex = args.struct_ident(EncodingType::LowerHex);
            tokens.extend(quote! {
                impl From<#lower_hex> for #ident {
                    fn from(value: #lower_hex) -> Self {
                        Self(value.to_uppercase())
                    }
                }
                impl From<&#lower_hex> for #ident {
                    fn from(value: &#lower_hex) -> Self {
                        Self(value.to_uppercase())
                    }
                }
            });
        }
        if args.encoding == EncodingType::LowerHex {
            let upper_hex = args.struct_ident(EncodingType::UpperHex);
            tokens.extend(quote! {
                impl From<#upper_hex> for #ident {
                    fn from(value: #upper_hex) -> Self {
                        Self(value.to_lowercase())
                    }
                }
                impl From<&#upper_hex> for #ident {
                    fn from(value: &#upper_hex) -> Self {
                        Self(value.to_lowercase())
                    }
                }
            });
        }

        // Eq/PartialEq
        //
        // Implementations should use [`constant_time_eq`]

        tokens.extend(quote! {
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
        });
        tokens
    }
}
