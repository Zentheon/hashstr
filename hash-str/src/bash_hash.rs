use bash_hash::{BashHash256, BashHash384, BashHash512};
use digest::consts::{U32, U48, U64};
use hash_str_derive::impl_hash_str;

impl_hash_str!(hasher = bash_hash::BashHash256, con = U32);

impl_hash_str!(hasher = bash_hash::BashHash384, con = U48);

impl_hash_str!(hasher = bash_hash::BashHash512, con = U64);
