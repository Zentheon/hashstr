#![doc = include_str!("../../README.md")]

use digest::consts::{U28, U32, U48, U64, U200};
use hash_strings_derive::impl_hash_string;
use sha3::{
    Keccak224, Keccak256, Keccak256Full, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384,
    Sha3_512,
};

impl_hash_string!(hasher = Sha3_224, con = U28);

impl_hash_string!(hasher = Sha3_256, con = U32);

impl_hash_string!(hasher = Sha3_384, con = U48);

impl_hash_string!(hasher = Sha3_512, con = U64);

impl_hash_string!(hasher = Keccak224, con = U28);

impl_hash_string!(hasher = Keccak256, con = U32);

impl_hash_string!(hasher = Keccak256Full, con = U200);

impl_hash_string!(hasher = Keccak384, con = U48);

impl_hash_string!(hasher = Keccak512, con = U64);
