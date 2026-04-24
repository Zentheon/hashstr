use crate::{EncodingError, Error};
use digest::consts::{U24, U28, U32, U48, U64};
use hash_strings_derive::impl_hash_string;
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};

impl_hash_string!(hasher = Shabal192, con = U24);

impl_hash_string!(hasher = Shabal224, con = U28);

impl_hash_string!(hasher = Shabal256, con = U32);

impl_hash_string!(hasher = Shabal384, con = U48);

impl_hash_string!(hasher = Shabal512, con = U64);
