use digest::consts::U20;
use hash_strings_derive::{StringDigest, StringWrapper};
use sha1::Sha1;

#[derive(StringDigest, StringWrapper)]
#[hash_string(hasher = Sha1, con = U20)]
pub struct Sha1String(pub String);
