use crate::{EncodingError, Error};
use blake2::{Blake2b512, Blake2s256, Digest, digest::generic_array::GenericArray};
use digest::consts::{U32, U64};
use hash_strings_derive::{StringDigest, StringWrapper};

#[derive(Clone, Debug, Eq, StringDigest, StringWrapper)]
#[hash_string(hasher = Blake2s256, con = U32, digest = Digest, no_io_wrapper)]
pub struct Blake2s256String(pub fstr::FStr<64>);

#[derive(Clone, Debug, Eq, StringDigest, StringWrapper)]
#[hash_string(hasher = Blake2b512, con = U64, digest = Digest, no_io_wrapper)]
pub struct Blake2b512String(pub fstr::FStr<128>);

impl From<&GenericArray<u8, U32>> for Blake2s256String {
    fn from(digest: &GenericArray<u8, U32>) -> Self {
        Self(base16ct::lower::encode_string(digest).try_into().unwrap())
    }
}

impl From<GenericArray<u8, U32>> for Blake2s256String {
    fn from(digest: GenericArray<u8, U32>) -> Self {
        Self::from(&digest)
    }
}

impl From<&GenericArray<u8, U64>> for Blake2b512String {
    fn from(digest: &GenericArray<u8, U64>) -> Self {
        Self(base16ct::lower::encode_string(digest).try_into().unwrap())
    }
}

impl From<GenericArray<u8, U64>> for Blake2b512String {
    fn from(digest: GenericArray<u8, U64>) -> Self {
        Self::from(&digest)
    }
}
