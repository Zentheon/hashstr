#![doc = include_str!("../../README.md")]

use digest::consts::{U28, U32, U48, U64};
use hash_strings_derive::impl_hash_string;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

impl_hash_string!(hasher = Sha224, con = U28);

impl_hash_string!(hasher = Sha256, con = U32);

impl_hash_string!(hasher = Sha384, con = U48);

impl_hash_string!(hasher = Sha512, con = U64);

impl_hash_string!(hasher = Sha512_224, con = U28);

impl_hash_string!(hasher = Sha512_256, con = U32);
