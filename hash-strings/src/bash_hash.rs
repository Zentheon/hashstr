use crate::{EncodingError, Error};
use bash_hash::{BashHash256, BashHash384, BashHash512};
use digest::consts::{U32, U48, U64};
use hash_strings_derive::impl_hash_string;

impl_hash_string!(hasher = BashHash256, con = U32);

impl_hash_string!(hasher = BashHash384, con = U48);

impl_hash_string!(hasher = BashHash512, con = U64);
