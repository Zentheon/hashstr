use digest::consts::U16;
use hashstr_derive::impl_hashstr;
use md5::Md5;

impl_hashstr!(hasher = md5::Md5, con = U16);
