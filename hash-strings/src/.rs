use crate::{EncodingError, Error};
use digest::consts::U64;
use hash_strings_derive::impl_hash_string;
use whirlpool::Whirlpool;

impl_hash_string!(hasher = Whirlpool, con = U64);
