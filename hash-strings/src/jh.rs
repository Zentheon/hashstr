use crate::{EncodingError, Error};
use digest::consts::{U28, U32, U48, U64};
use hash_strings_derive::impl_hash_string;
use jh::{Jh224, Jh256, Jh384, Jh512};

impl_hash_string!(hasher = Jh224, con = U28);

impl_hash_string!(hasher = Jh256, con = U32);

impl_hash_string!(hasher = Jh384, con = U48);

impl_hash_string!(hasher = Jh512, con = U64);
