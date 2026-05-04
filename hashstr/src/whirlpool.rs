use digest::consts::U64;
use hashstr_derive::impl_hashstr;
use whirlpool::Whirlpool;

impl_hashstr!(hasher = whirlpool::Whirlpool, con = U64);
