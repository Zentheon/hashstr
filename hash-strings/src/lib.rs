#![doc = include_str!("../../README.md")]

// Hashers with custom code:
// blake2
// blake3
// sha1_checked

use std::fmt::Display;

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

#[derive(Debug, Clone)]
pub struct LengthError {
    pub expected: usize,
    pub actual: usize,
    pub hash_name: String,
}

#[derive(Debug, Clone)]
pub struct EncodingError {
    pub hash_name: String,
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
    pub fn from_fstr_err<T>(res: Result<T, fstr::LengthError>, name: &str) -> Result<T, Error> {
        match res {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::LengthError(LengthError {
                expected: e.expected(),
                actual: e.actual(),
                hash_name: name.to_string(),
            })),
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
