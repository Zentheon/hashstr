use belt_hash::BeltHash;
use digest::consts::U32;
use hash_strings_derive::impl_hash_string;

impl_hash_string!(hasher = belt_hash::BeltHash, con = U32);
