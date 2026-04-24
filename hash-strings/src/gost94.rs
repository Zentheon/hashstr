use crate::{EncodingError, Error};
use digest::consts::U32;
use gost94::{Gost94CryptoPro, Gost94Test, Gost94UA, Gost94s2015};
use hash_strings_derive::impl_hash_string;

impl_hash_string!(hasher = Gost94CryptoPro, con = U32);

impl_hash_string!(hasher = Gost94Test, con = U32);

impl_hash_string!(hasher = Gost94UA, con = U32);

impl_hash_string!(hasher = Gost94s2015, con = U32);
