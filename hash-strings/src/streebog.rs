use digest::consts::{U32, U64};
use hash_strings_derive::impl_hash_string;
use streebog::{Streebog256, Streebog512};

impl_hash_string!(hasher = streebog::Streebog256, con = U32);

impl_hash_string!(hasher = streebog::Streebog512, con = U64);
