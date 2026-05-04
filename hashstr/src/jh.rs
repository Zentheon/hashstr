use digest::consts::{U28, U32, U48, U64};
use hashstr_derive::impl_hashstr;
use jh::{Jh224, Jh256, Jh384, Jh512};

impl_hashstr!(hasher = jh::Jh224, con = U28);

impl_hashstr!(hasher = jh::Jh256, con = U32);

impl_hashstr!(hasher = jh::Jh384, con = U48);

impl_hashstr!(hasher = jh::Jh512, con = U64);
