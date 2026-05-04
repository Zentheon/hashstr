use digest::consts::U16;
use hashstr_derive::impl_hashstr;
use md4::Md4;

impl_hashstr!(hasher = md4::Md4, con = U16);
