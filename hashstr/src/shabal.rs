use digest::consts::{U24, U28, U32, U48, U64};
use hashstr_derive::impl_hashstr;
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};

impl_hashstr!(hasher = shabal::Shabal192, con = U24);

impl_hashstr!(hasher = shabal::Shabal224, con = U28);

impl_hashstr!(hasher = shabal::Shabal256, con = U32);

impl_hashstr!(hasher = shabal::Shabal384, con = U48);

impl_hashstr!(hasher = shabal::Shabal512, con = U64);
