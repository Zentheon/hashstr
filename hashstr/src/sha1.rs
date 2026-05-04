use digest::consts::U20;
use hashstr_derive::impl_hashstr;
use sha1::Sha1;

impl_hashstr!(hasher = sha1::Sha1, con = U20);
