use digest::consts::U20;
use hash_str_derive::impl_hash_str;
use sha1::Sha1;

impl_hash_str!(hasher = sha1::Sha1, con = U20);
