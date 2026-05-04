use ascon_hash256::AsconHash256;
use digest::consts::U32;
use hashstr_derive::impl_hashstr;

impl_hashstr!(hasher = ascon_hash256::AsconHash256, con = U32);
