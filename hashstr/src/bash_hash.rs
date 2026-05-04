use bash_hash::{BashHash256, BashHash384, BashHash512};
use digest::consts::{U32, U48, U64};
use hashstr_derive::impl_hashstr;

impl_hashstr!(hasher = bash_hash::BashHash256, con = U32);

impl_hashstr!(hasher = bash_hash::BashHash384, con = U48);

impl_hashstr!(hasher = bash_hash::BashHash512, con = U64);
