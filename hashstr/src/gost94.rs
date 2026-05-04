use digest::consts::U32;
use gost94::{Gost94CryptoPro, Gost94Test, Gost94UA, Gost94s2015};
use hashstr_derive::impl_hashstr;

impl_hashstr!(hasher = gost94::Gost94CryptoPro, con = U32);

impl_hashstr!(hasher = gost94::Gost94Test, con = U32);

impl_hashstr!(hasher = gost94::Gost94UA, con = U32);

impl_hashstr!(hasher = gost94::Gost94s2015, con = U32);
