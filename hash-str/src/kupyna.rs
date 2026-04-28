use digest::consts::{U28, U32, U48, U64};
use hash_str_derive::impl_hash_str;
use kupyna::{Kupyna224, Kupyna256, Kupyna384, Kupyna512};

impl_hash_str!(hasher = kupyna::Kupyna224, con = U28);

impl_hash_str!(hasher = kupyna::Kupyna256, con = U32);

impl_hash_str!(hasher = kupyna::Kupyna384, con = U48);

impl_hash_str!(hasher = kupyna::Kupyna512, con = U64);
