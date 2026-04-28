use digest::consts::U16;
use hash_str_derive::impl_hash_str;
use md5::Md5;

impl_hash_str!(hasher = md5::Md5, con = U16);
