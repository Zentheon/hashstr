use digest::consts::{U20, U28, U32, U48, U64};
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
use hashstr_derive::impl_hashstr;

impl_hashstr!(hasher = fsb::Fsb160, con = U20);

impl_hashstr!(hasher = fsb::Fsb224, con = U28);

impl_hashstr!(hasher = fsb::Fsb256, con = U32);

impl_hashstr!(hasher = fsb::Fsb384, con = U48);

impl_hashstr!(hasher = fsb::Fsb512, con = U64);
