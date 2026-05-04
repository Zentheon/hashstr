use digest::consts::{U32, U64};
use hashstr_derive::impl_hashstr;
use streebog::{Streebog256, Streebog512};

impl_hashstr!(hasher = streebog::Streebog256, con = U32);

impl_hashstr!(hasher = streebog::Streebog512, con = U64);
