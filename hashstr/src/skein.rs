use digest::consts::{U32, U64, U128};
use hashstr_derive::impl_hashstr;
use skein::{
    Skein256_256, Skein256_512, Skein512_256, Skein512_512, Skein1024_256, Skein1024_512,
    Skein1024_1024,
};

impl_hashstr!(hasher = skein::Skein256_256, con = U32);

impl_hashstr!(hasher = skein::Skein256_512, con = U64);

impl_hashstr!(hasher = skein::Skein512_256, con = U32);

impl_hashstr!(hasher = skein::Skein512_512, con = U64);

impl_hashstr!(hasher = skein::Skein1024_256, con = U32);

impl_hashstr!(hasher = skein::Skein1024_512, con = U64);

impl_hashstr!(hasher = skein::Skein1024_1024, con = U128);
