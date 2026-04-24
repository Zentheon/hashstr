use crate::{EncodingError, Error};
use digest::consts::{U28, U32, U48, U64};
use hash_strings_derive::impl_hash_string;
use kupyna::{Kupyna224, Kupyna256, Kupyna384, Kupyna512};

impl_hash_string!(hasher = Kupyna224, con = U28);

impl_hash_string!(hasher = Kupyna256, con = U32);

impl_hash_string!(hasher = Kupyna384, con = U48);

impl_hash_string!(hasher = Kupyna512, con = U64);
