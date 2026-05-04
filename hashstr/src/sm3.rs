use digest::consts::U32;
use hashstr_derive::impl_hashstr;
use sm3::Sm3;

impl_hashstr!(hasher = sm3::Sm3, con = U32);
