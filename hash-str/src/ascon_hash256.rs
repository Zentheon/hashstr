use ascon_hash256::AsconHash256;
use digest::consts::U32;
use hash_str_derive::impl_hash_str;

impl_hash_str!(hasher = ascon_hash256::AsconHash256, con = U32);
