use belt_hash::BeltHash;
use digest::consts::U32;
use hash_str_derive::impl_hash_str;

impl_hash_str!(hasher = belt_hash::BeltHash, con = U32);
