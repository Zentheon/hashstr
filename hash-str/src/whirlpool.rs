use digest::consts::U64;
use hash_str_derive::impl_hash_str;
use whirlpool::Whirlpool;

impl_hash_str!(hasher = whirlpool::Whirlpool, con = U64);
