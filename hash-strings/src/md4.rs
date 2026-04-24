use crate::{EncodingError, Error};
use digest::consts::U16;
use hash_strings_derive::impl_hash_string;
use md4::Md4;

impl_hash_string!(hasher = Md4, con = U16);
