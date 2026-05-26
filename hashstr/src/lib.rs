#![doc = include_str!("../README.md")]
// Hashers with custom code:
// blake2
// blake3
// sha1_checked
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
#[macro_use]
extern crate std;

use fstr::FStr;

#[cfg(feature = "ascon-hash256")]
pub mod ascon_hash256;
#[cfg(feature = "bash-hash")]
pub mod bash_hash;
#[cfg(feature = "belt-hash")]
pub mod belt_hash;
#[cfg(feature = "blake2")]
pub mod blake2;
#[cfg(feature = "blake3")]
pub mod blake3;
#[cfg(feature = "fsb")]
pub mod fsb;
#[cfg(feature = "gost94")]
pub mod gost94;
#[cfg(feature = "groestl")]
pub mod groestl;
#[cfg(feature = "jh")]
pub mod jh;
#[cfg(feature = "kupyna")]
pub mod kupyna;
#[cfg(feature = "md2")]
pub mod md2;
#[cfg(feature = "md4")]
pub mod md4;
#[cfg(feature = "md5")]
pub mod md5;
#[cfg(feature = "ripemd")]
pub mod ripemd;
#[cfg(feature = "sha1")]
pub mod sha1;
#[cfg(feature = "sha1-checked")]
pub mod sha1_checked;
#[cfg(feature = "sha2")]
pub mod sha2;
#[cfg(feature = "sha3")]
pub mod sha3;
#[cfg(feature = "shabal")]
pub mod shabal;
#[cfg(feature = "skein")]
pub mod skein;
#[cfg(feature = "sm3")]
pub mod sm3;
#[cfg(feature = "streebog")]
pub mod streebog;
#[cfg(feature = "tiger")]
pub mod tiger;
#[cfg(feature = "whirlpool")]
pub mod whirlpool;

/// The table of lowercase letters (no numbers).
pub const HEX_LETTERS_LOWER: &[u8; 6] = b"abcdef";

/// The table of uppercase letters (no numbers).
pub const HEX_LETTERS_UPPER: &[u8; 6] = b"ABCDEF";

/// Takes a number of hexadecimal bytes and converts their casing.
///
/// This function only finds letters 'A' through 'F' and case-swaps them; it does not care if the
/// input is properly encoded hex or not.
pub const fn convert_hex_case<const HEX_LEN: usize, const TO_UPPER: bool>(
    hex: &[u8; HEX_LEN],
) -> [u8; HEX_LEN] {
    // Initial vars
    let (search, replace) = if TO_UPPER {
        (HEX_LETTERS_LOWER, HEX_LETTERS_UPPER)
    } else {
        (HEX_LETTERS_UPPER, HEX_LETTERS_LOWER)
    };
    debug_assert!(search.len() == replace.len());
    let lookup_end = search.len() - 1;

    let mut converted = [0u8; HEX_LEN];
    let mut pos = 0;
    let mut lookup = 0;

    // Loop over chars in hex input
    while pos < HEX_LEN {
        // Try to find casing matches that should be swapped
        while lookup <= lookup_end {
            if hex[pos] == search[lookup] {
                converted[pos] = replace[lookup];
                break;
            }
            // End of lookup options: Copy current char
            if lookup == lookup_end {
                converted[pos] = hex[pos];
            }
            lookup += 1;
        }
        lookup = 0;
        pos += 1;
    }
    converted
}

pub const fn convert_hex_case_fstr<const HEX_LEN: usize, const TO_UPPER: bool>(
    hex: &FStr<HEX_LEN>,
) -> FStr<HEX_LEN> {
    let hex_array = convert_hex_case::<HEX_LEN, TO_UPPER>(hex.as_bytes());
    unsafe { FStr::from_inner_unchecked(hex_array) }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use std::string::String;
    use std::string::ToString;
    use tracing::info;

    #[test]
    #[tracing_test::traced_test]
    fn test_convert_hex_case_fstr() {
        let hex1: FStr<6> = FStr::from_str_const("f2ad9a");
        let hex2: FStr<12> = FStr::from_str_const("adFcAaaBCCfd");
        let hex3: FStr<12> = FStr::from_str_const("AABBCCDDEEFF");

        info!("hex1: {hex1}");
        info!("hex2: {hex2}");
        info!("hex3: {hex3}");
        info!("hex array: {:?}", hex3.as_bytes().as_array::<12>().unwrap());

        let lower_array = convert_hex_case::<12, true>(hex3.as_bytes());
        let lower1 = convert_hex_case_fstr::<6, false>(&hex1);
        let upper1 = convert_hex_case_fstr::<6, true>(&hex1);

        info!("lower1: {lower1}");
        info!("hex array to lower: {:?}", lower_array);

        assert!(upper1 != lower1);
        assert!(upper1 == lower1.to_uppercase());
    }
}

/// Encodes a byte slice to an [`FStr`] in hexadecimal
///
/// # Args
/// * `value`: The data to encode.
/// * `upper`: Encodes to uppercase hex if `true`.
/// * `hash_name`: Name of the hasher to use in an error.
///
/// # Errors
/// * [`Error::LengthError`]: If `value.len()` is does not equal `N * 2`
pub fn encode_hex<const HEX_LEN: usize>(
    value: impl AsRef<[u8]>,
    upper: bool,
    hash_name: &'static str,
) -> Result<FStr<HEX_LEN>, Error> {
    let value = value.as_ref();
    if value.len() != HEX_LEN / 2 {
        Err(Error::LengthError(LengthError {
            expected: HEX_LEN / 2,
            actual: value.len(),
            hash_name,
        }))?;
    };

    let mut hex = [0u8; HEX_LEN];

    let _ = if upper {
        base16ct::upper::encode(value, &mut hex)
    } else {
        base16ct::lower::encode(value, &mut hex)
    };

    unsafe { Ok(FStr::from_inner_unchecked(hex)) }
}

/// Decodes hexadecimal into raw bytes using [`base16ct`]
///
/// # Args
/// * `const N`: Expected length after decode
/// * `value`: The data to encode.
/// * `hash_name`: Name of the hasher to use in an error.
///
/// # Errors
/// * [`Error::HexError`]: If the input did not contain valid base16
pub fn decode_hex<const N: usize, const UPPER: bool>(
    value: impl AsRef<[u8]>,
    hash_name: &'static str,
) -> Result<[u8; N], Error> {
    let mut decoded = [0u8; N];

    match if UPPER {
        base16ct::upper::decode(&value, &mut decoded)
    } else {
        base16ct::lower::decode(&value, &mut decoded)
    } {
        Ok(_) => Ok(()),
        Err(base16ct::Error::InvalidEncoding) => {
            Err(Error::EncodingError(crate::EncodingError { hash_name }))
        }
        Err(base16ct::Error::InvalidLength) => Err(Error::LengthError(LengthError {
            expected: N,
            actual: value.as_ref().len(),
            hash_name,
        })),
    }?;
    Ok(decoded)
}

/// Builds an [`Error`] if the input value length is not `N`
pub const fn check_len<const N: usize>(value: &[u8], hash_name: &'static str) -> Result<(), Error> {
    let actual = value.len();
    if actual == N {
        Ok(())
    } else {
        Err(Error::LengthError(LengthError {
            expected: N,
            actual,
            hash_name,
        }))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct LengthError {
    pub(crate) expected: usize,
    pub(crate) actual: usize,
    pub(crate) hash_name: &'static str,
}

impl LengthError {
    pub const fn expected(&self) -> usize {
        self.expected
    }
    pub const fn actual(&self) -> usize {
        self.actual
    }
    pub const fn hash_name(&self) -> &'static str {
        self.hash_name
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct EncodingError {
    pub(crate) hash_name: &'static str,
}

impl EncodingError {
    pub const fn hash_name(&self) -> &'static str {
        self.hash_name
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    LengthError(LengthError),
    EncodingError(EncodingError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LengthError(e) => write!(f, "{e}"),
            Self::EncodingError(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for LengthError {}
impl core::error::Error for EncodingError {}
impl core::error::Error for Error {}

impl core::fmt::Display for LengthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Invalid {} length of {} (expected: {})",
            self.hash_name, self.expected, self.actual
        )
    }
}

impl core::fmt::Display for EncodingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Characters in {} should be hexadecimal", self.hash_name)
    }
}
