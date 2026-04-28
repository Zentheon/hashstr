use digest::consts::{U32, U64};
use hash_str_derive::impl_hash_str;
use streebog::{Streebog256, Streebog512};

impl_hash_str!(hasher = streebog::Streebog256, con = U32);

impl_hash_str!(hasher = streebog::Streebog512, con = U64);
