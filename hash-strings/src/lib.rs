use std::fmt::Display;

use fstr::LengthError;

#[cfg(feature = "blake2")]
pub mod blake2;
#[cfg(feature = "blake3")]
pub mod blake3;
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
pub struct LenError {
    pub expected: usize,
    pub got: usize,
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

impl From<LengthError> for Error {
    fn from(value: LengthError) -> Self {
        Error::LengthError(value)
    }
}

impl std::error::Error for LenError {}
impl std::error::Error for EncodingError {}
impl std::error::Error for Error {}

impl Display for LenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid length of {}. Expected: {}, got: {}",
            self.hash_name, self.expected, self.got
        )
    }
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Characters in {} should be hexadecimal", self.hash_name)
    }
}
