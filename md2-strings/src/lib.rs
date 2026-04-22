use digest::consts::U16;
use hash_strings_derive::HashString;
use md2::Md2;

#[derive(HashString)]
#[hash_string(hasher = Md2, con = U16)]
pub struct Md2String(pub String);
