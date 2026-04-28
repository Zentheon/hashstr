use digest::consts::U16;
use hash_str_derive::impl_hash_str;
use md4::Md4;

impl_hash_str!(hasher = md4::Md4, con = U16);
