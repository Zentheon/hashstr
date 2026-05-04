use digest::consts::{U28, U32, U48, U64};
use hashstr_derive::impl_hashstr;
use kupyna::{Kupyna224, Kupyna256, Kupyna384, Kupyna512};

impl_hashstr!(hasher = kupyna::Kupyna224, con = U28);

impl_hashstr!(hasher = kupyna::Kupyna256, con = U32);

impl_hashstr!(hasher = kupyna::Kupyna384, con = U48);

impl_hashstr!(hasher = kupyna::Kupyna512, con = U64);
