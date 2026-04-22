#![doc = include_str!("../../README.md")]

use digest::consts::{U28, U32, U48, U64};
use hash_strings_derive::{StringDigest, StringWrapper};
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha224, con = U28)]
pub struct Sha224String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha256, con = U32)]
pub struct Sha256String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha384, con = U48)]
pub struct Sha384String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha512, con = U64)]
pub struct Sha512String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha512_224, con = U28)]
pub struct Sha512_224String(pub String);

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha512_256, con = U32)]
pub struct Sha512_256String(pub String);
