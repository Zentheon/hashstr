use digest::consts::U16;
use hashstr_derive::impl_hashstr;
use md2::Md2;

impl_hashstr!(hasher = md2::Md2, con = U16);
