use ascon_hash256::AsconHash256;
use digest::consts::U32;
use hash_strings_derive::impl_hash_string;

impl_hash_string!(hasher = ascon_hash256::AsconHash256, con = U32);
