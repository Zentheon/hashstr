use digest::consts::U16;
use hash_strings_derive::impl_hash_string;
use md2::Md2;

impl_hash_string!(hasher = md2::Md2, con = U16);
