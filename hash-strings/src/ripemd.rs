use crate::{EncodingError, Error};
use digest::consts::{U16, U20, U32, U40};
use hash_strings_derive::impl_hash_string;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};

impl_hash_string!(hasher = Ripemd128, con = U16);

impl_hash_string!(hasher = Ripemd160, con = U20);

impl_hash_string!(hasher = Ripemd256, con = U32);

impl_hash_string!(hasher = Ripemd320, con = U40);
