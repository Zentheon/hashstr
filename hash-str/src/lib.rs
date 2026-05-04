#![doc = include_str!("../README.md")]

// Hashers with custom code:
// blake2
// blake3
// sha1_checked

use std::fmt::Display;

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

/// Takes a number of hexadecimal bytes and converts uppercase letters to lowercase.
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

#[test]
fn test_convert_hex_case_fstr() {
    let hex1: FStr<6> = FStr::from_str_unwrap("f2ad9a");
    let hex2: FStr<12> = FStr::from_str_unwrap("adFcAaaBCCfd");
    let hex3: FStr<12> = FStr::from_str_unwrap("AABBCCDDEEFF");

    println!("hex1: {hex1}");
    println!("hex2: {hex2}");
    println!("hex3: {hex3}");
    println!("hex array: {:?}", hex3.as_bytes().as_array::<12>().unwrap());

    let lower_array = convert_hex_case::<12, true>(&hex3.as_bytes());
    let lower1 = convert_hex_case_fstr::<6, false>(&hex1);
    let upper1 = convert_hex_case_fstr::<6, true>(&hex1);

    println!("lower1: {lower1}");
    println!("hex array to lower: {:?}", lower_array);

    assert!(upper1 != lower1);
    assert!(upper1 == lower1.to_uppercase());
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
    let mut hex = [0u8; HEX_LEN];

    let encode = if upper {
        const_hex::encode_to_slice_upper(value, &mut hex)
    } else {
        const_hex::encode_to_slice(value, &mut hex)
    };
    match encode {
        // SAFETY: Must be valid utf-8. Should already be well within bounds after the hex
        // encode
        Ok(_) => Ok(unsafe { FStr::from_inner_unchecked(hex) }),
        Err(err) => Err(Error::from_hex_err(
            err,
            HEX_LEN / 2,
            value.len(),
            hash_name,
        )),
    }
}

/// Encodes a byte slice to an [`FStr`] in lowercase hexidecimal
pub const fn encode_hex_const<const HEX_LEN: usize, const UPPER: bool>(
    value: &[u8; HEX_LEN],
) -> FStr<HEX_LEN> {
    let buf: const_hex::Buffer<HEX_LEN> = if UPPER {
        const_hex::Buffer::new().const_format_upper(value)
    } else {
        const_hex::Buffer::new().const_format(value)
    };
    // SAFETY: Must be valid utf-8. Should already be well within bounds after the hex
    // encode
    unsafe { FStr::from_inner_unchecked(*buf.as_byte_array()) }
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct HexError {
    pub(crate) char: char,
    pub(crate) index: usize,
    pub(crate) hash_name: &'static str,
}

impl HexError {
    pub const fn char(&self) -> char {
        self.char
    }
    pub const fn index(&self) -> usize {
        self.index
    }
    pub const fn hash_name(&self) -> &'static str {
        self.hash_name
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    LengthError(LengthError),
    HexError(HexError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthError(e) => write!(f, "{e}"),
            Self::HexError(e) => write!(f, "{e}"),
        }
    }
}

impl Error {
    pub fn from_fstr_err<T>(
        res: Result<T, fstr::LengthError>,
        name: &'static str,
    ) -> Result<T, Error> {
        match res {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::LengthError(LengthError {
                expected: e.expected(),
                actual: e.actual(),
                hash_name: name,
            })),
        }
    }
    pub const fn from_hex_err(
        err: const_hex::FromHexError,
        expected: usize,
        actual: usize,
        hash_name: &'static str,
    ) -> Error {
        match err {
            const_hex::FromHexError::InvalidHexCharacter { c, index } => {
                Error::HexError(HexError {
                    char: c,
                    index,
                    hash_name,
                })
            }
            const_hex::FromHexError::OddLength => Error::LengthError(LengthError {
                expected: expected,
                actual: actual,
                hash_name,
            }),
            const_hex::FromHexError::InvalidStringLength => Error::LengthError(LengthError {
                expected: expected,
                actual: actual,
                hash_name,
            }),
        }
    }
}

impl std::error::Error for LengthError {}
impl std::error::Error for HexError {}
impl std::error::Error for Error {}

impl Display for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid {} length of {} (expected: {})",
            self.hash_name, self.expected, self.actual
        )
    }
}

impl Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Characters in {} should be hexadecimal", self.hash_name)
    }
}
