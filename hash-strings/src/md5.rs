use crate::{EncodingError, Error};
use digest::consts::U16;
use hash_strings_derive::impl_hash_string;
use md5::Md5;

impl_hash_string!(hasher = Md5, con = U16);
