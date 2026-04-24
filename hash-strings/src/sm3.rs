use crate::{EncodingError, Error};
use digest::consts::U32;
use hash_strings_derive::impl_hash_string;
use sm3::Sm3;

impl_hash_string!(hasher = Sm3, con = U32);
