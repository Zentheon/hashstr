use digest::consts::U16;
use hash_strings_derive::{StringDigest, StringWrapper};
use md4::Md4;

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Md4, con = U16)]
pub struct Md4String(pub String);
