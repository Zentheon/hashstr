use digest::consts::{U32, U64, U128};
use hash_strings_derive::impl_hash_string;
use skein::{
    Skein256_256, Skein256_512, Skein512_256, Skein512_512, Skein1024_256, Skein1024_512,
    Skein1024_1024,
};

impl_hash_string!(hasher = skein::Skein256_256, con = U32);

impl_hash_string!(hasher = skein::Skein256_512, con = U64);

impl_hash_string!(hasher = skein::Skein512_256, con = U32);

impl_hash_string!(hasher = skein::Skein512_512, con = U64);

impl_hash_string!(hasher = skein::Skein1024_256, con = U32);

impl_hash_string!(hasher = skein::Skein1024_512, con = U64);

impl_hash_string!(hasher = skein::Skein1024_1024, con = U128);
