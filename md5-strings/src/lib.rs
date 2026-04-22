use digest::consts::U16;
use hash_strings_derive::HashString;
use md5::Md5;

#[derive(HashString)]
#[hash_string(hasher = Md5, con = U16)]
pub struct Md5String(pub String);
