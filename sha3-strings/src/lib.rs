#![doc = include_str!("../../README.md")]

use digest::consts::{U28, U32, U48, U64, U200};
use hash_strings_derive::{StringDigest, StringWrapper};
use sha3::{
    Keccak224, Keccak256, Keccak256Full, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384,
    Sha3_512,
};

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha3_224, con = U28)]
pub struct Sha3_224String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha3_256, con = U32)]
pub struct Sha3_256String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha3_384, con = U48)]
pub struct Sha3_384String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha3_512, con = U64)]
pub struct Sha3_512String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Keccak224, con = U28)]
pub struct Keccak224String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Keccak256, con = U32)]
pub struct Keccak256String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Keccak256Full, con = U200)]
pub struct Keccak256FullString(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Keccak384, con = U48)]
pub struct Keccak384String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Keccak512, con = U64)]
pub struct Keccak512String(pub String);
