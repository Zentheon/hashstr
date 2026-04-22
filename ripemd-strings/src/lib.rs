use digest::consts::{U16, U20, U32, U40};
use hash_strings_derive::HashString;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};

#[derive(HashString)]
#[hash_string(hasher = Ripemd128, con = U16)]
pub struct Ripemd128String(pub String);

#[derive(HashString)]
#[hash_string(hasher = Ripemd160, con = U20)]
pub struct Ripemd160String(pub String);

#[derive(HashString)]
#[hash_string(hasher = Ripemd256, con = U32)]
pub struct Ripemd256String(pub String);

#[derive(HashString)]
#[hash_string(hasher = Ripemd320, con = U40)]
pub struct Ripemd320String(pub String);
