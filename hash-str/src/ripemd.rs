use digest::consts::{U16, U20, U32, U40};
use hash_str_derive::impl_hash_str;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};

impl_hash_str!(hasher = ripemd::Ripemd128, con = U16);

impl_hash_str!(hasher = ripemd::Ripemd160, con = U20);

impl_hash_str!(hasher = ripemd::Ripemd256, con = U32);

impl_hash_str!(hasher = ripemd::Ripemd320, con = U40);
