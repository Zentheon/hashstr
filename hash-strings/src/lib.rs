#![doc = include_str!("../../README.md")]

// Hashers with custom code:
// blake2
// blake3
// sha1_checked

use std::fmt::Display;

use const_hex::encode_to_slice;
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

/// Encodes a byte slice to an [`FStr`] in lowercase hexidecminal
pub fn encode_lower_hex<const N: usize>(
    value: impl AsRef<[u8]>,
    hash_name: &'static str,
) -> Result<FStr<N>, Error> {
    let value = value.as_ref();
    let mut hex = [0u8; _];

    assert!(hex.len() == 2 * value.len());
    match encode_to_slice(value, &mut hex) {
        // SAFETY: Must be valid utf-8. Should already be well within bounds after the hex
        // encode
        Ok(_) => Ok(unsafe { FStr::from_inner_unchecked(hex) }),
        Err(err) => Err(Error::from_hex_err::<N>(err, value.len(), hash_name)),
    }
}

/// Encodes a byte slice to an [`FStr`] in lowercase hexidecminal
pub const fn encode_lower_hex_const<const N: usize>(value: &[u8; N]) -> FStr<N> {
    let buf = const_hex::const_encode::<N, false>(value);
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

#[derive(Debug, Clone)]
pub struct EncodingError {
    pub(crate) c: char,
    pub(crate) index: usize,
    pub(crate) hash_name: &'static str,
}

#[derive(Debug, Clone)]
pub enum Error {
    LengthError(LengthError),
    EncodingError(EncodingError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthError(e) => write!(f, "{e}"),
            Self::EncodingError(e) => write!(f, "{e}"),
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
    pub const fn from_hex_err<const N: usize>(
        err: const_hex::FromHexError,
        len: usize,
        hash_name: &'static str,
    ) -> Error {
        match err {
            const_hex::FromHexError::InvalidHexCharacter { c, index } => {
                Error::EncodingError(EncodingError {
                    c,
                    index,
                    hash_name,
                })
            }
            const_hex::FromHexError::OddLength => Error::LengthError(LengthError {
                expected: N,
                actual: len,
                hash_name,
            }),
            const_hex::FromHexError::InvalidStringLength => Error::LengthError(LengthError {
                expected: N,
                actual: len,
                hash_name,
            }),
        }
    }
}

impl std::error::Error for LengthError {}
impl std::error::Error for EncodingError {}
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

impl Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Characters in {} should be hexadecimal", self.hash_name)
    }
}
