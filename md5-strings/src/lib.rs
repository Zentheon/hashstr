use digest::consts::U16;
use hash_strings_derive::{StringDigest, StringWrapper};
use md5::Md5;

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Md5, con = U16)]
pub struct Md5String(pub String);
