use crate::{EncodingError, Error};
use belt_hash::BeltHash;
use digest::consts::U32;
use hash_strings_derive::impl_hash_string;

impl_hash_string!(hasher = BeltHash, con = U32);
