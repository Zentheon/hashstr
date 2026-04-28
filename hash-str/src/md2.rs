use digest::consts::U16;
use hash_str_derive::impl_hash_str;
use md2::Md2;

impl_hash_str!(hasher = md2::Md2, con = U16);
