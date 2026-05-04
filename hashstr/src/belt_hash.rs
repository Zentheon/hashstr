use belt_hash::BeltHash;
use digest::consts::U32;
use hashstr_derive::impl_hashstr;

impl_hashstr!(hasher = belt_hash::BeltHash, con = U32);
