use digest::consts::{U28, U32, U48, U64};
use groestl::{Groestl224, Groestl256, Groestl384, Groestl512};
use hash_str_derive::impl_hash_str;

impl_hash_str!(hasher = groestl::Groestl224, con = U28);

impl_hash_str!(hasher = groestl::Groestl256, con = U32);

impl_hash_str!(hasher = groestl::Groestl384, con = U48);

impl_hash_str!(hasher = groestl::Groestl512, con = U64);
