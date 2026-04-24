use crate::{EncodingError, Error};
use digest::consts::U20;
use hash_strings_derive::impl_hash_string;
use sha1::Sha1;

impl_hash_string!(hasher = Sha1, con = U20);
